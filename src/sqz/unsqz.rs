use crate::error::{ArchiveError, Result};

// =====================================================================
// Bit reader (MSB first, like LHA/LZHUF)
// =====================================================================

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    bitbuf: u32,
    bits: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        let mut br = Self {
            data,
            pos: 0,
            bitbuf: 0,
            bits: 0,
        };
        // Pre-fill buffer
        let _ = br.fill_buf();
        br
    }

    fn fill_buf(&mut self) -> Result<()> {
        while self.bits <= 24 && self.pos < self.data.len() {
            self.bitbuf = (self.bitbuf << 8) | (self.data[self.pos] as u32);
            self.pos += 1;
            self.bits += 8;
        }
        Ok(())
    }

    fn peek_bits(&mut self, n: u8) -> u16 {
        if n == 0 {
            return 0;
        }
        let _ = self.fill_buf();
        if self.bits < n {
            return 0;
        }
        ((self.bitbuf >> (self.bits - n)) & ((1u32 << n) - 1)) as u16
    }

    fn skip_bits(&mut self, n: u8) {
        if n >= self.bits {
            self.bits = 0;
            self.bitbuf = 0;
        } else {
            self.bits -= n;
            if self.bits < 32 {
                self.bitbuf &= (1u32 << self.bits) - 1;
            }
        }
        let _ = self.fill_buf();
    }

    fn get_bits(&mut self, n: u8) -> u16 {
        let v = self.peek_bits(n);
        self.skip_bits(n);
        v
    }
}

// =====================================================================
// Huffman table decoding (table-based, like LZHUF)
// =====================================================================

const NC: usize = 0x1ff;  // 511: Number of C codes (literals + length codes)
const NT: usize = 0x13;   // 19: Number of PT codes  
const NP: usize = 0x1f;   // 31: Number of position codes
const TBIT: u8 = 5;       // Bits for PT table size
const CBIT: u8 = 9;       // Bits for C table size
const PBIT: u8 = 5;       // Bits for P table size

/// Tree-based Huffman decoder that handles incomplete trees
/// (required for SQZ format which can have unassigned code prefixes)
struct HuffTree {
    nodes: Vec<HuffNode>,
}

const NO_CHILD: usize = usize::MAX;

#[derive(Clone)]
enum HuffNode {
    Branch { left: usize, right: usize },
    Leaf(u16),
}

impl HuffTree {
    fn from_bit_lengths(bit_lengths: &[u8]) -> Result<Self> {
        // Find max length
        let max_len = bit_lengths.iter().copied().max().unwrap_or(0);
        if max_len == 0 {
            // All zeros - create a trivial tree that returns 0
            let nodes = vec![HuffNode::Leaf(0)];
            return Ok(HuffTree { nodes });
        }
        
        // Count codes per length
        let mut bl_count = vec![0u32; (max_len as usize) + 1];
        for &len in bit_lengths {
            if len > 0 {
                bl_count[len as usize] += 1;
            }
        }
        
        // Generate canonical codes
        let mut next_code = vec![0u32; (max_len as usize) + 1];
        let mut code = 0u32;
        for bits in 1..=(max_len as usize) {
            code = (code + bl_count[bits - 1]) << 1;
            next_code[bits] = code;
        }
        
        // Build tree - use NO_CHILD to indicate missing branches
        let mut nodes = vec![HuffNode::Branch { left: NO_CHILD, right: NO_CHILD }];
        
        for (sym, &len) in bit_lengths.iter().enumerate() {
            if len == 0 {
                continue;
            }
            let len_usize = len as usize;
            let sym_code = next_code[len_usize];
            next_code[len_usize] += 1;
            
            // Walk down tree, creating nodes as needed
            let mut node_idx = 0usize;
            for bit_idx in (0..len_usize).rev() {
                let bit = ((sym_code >> bit_idx) & 1) as usize;
                
                let (left, right) = match &nodes[node_idx] {
                    HuffNode::Branch { left, right } => (*left, *right),
                    HuffNode::Leaf(_) => {
                        // Shouldn't happen with valid codes
                        return Err(ArchiveError::decompression_failed(
                            "SQZ",
                            "Invalid Huffman tree structure",
                        ));
                    }
                };
                
                let child = if bit == 0 { left } else { right };
                if child == NO_CHILD {
                    // Need to create new node
                    let new_idx = nodes.len();
                    let new_node = if bit_idx == 0 {
                        HuffNode::Leaf(sym as u16)
                    } else {
                        HuffNode::Branch { left: NO_CHILD, right: NO_CHILD }
                    };
                    nodes.push(new_node);
                    
                    // Update parent
                    match &mut nodes[node_idx] {
                        HuffNode::Branch { left, right } => {
                            if bit == 0 {
                                *left = new_idx;
                            } else {
                                *right = new_idx;
                            }
                        }
                        HuffNode::Leaf(_) => unreachable!(),
                    }
                    node_idx = new_idx;
                } else {
                    node_idx = child;
                }
            }
        }
        
        Ok(HuffTree { nodes })
    }
    
    fn decode(&self, br: &mut BitReader) -> u16 {
        let mut idx = 0usize;
        
        loop {
            match &self.nodes.get(idx) {
                Some(HuffNode::Leaf(sym)) => return *sym,
                Some(HuffNode::Branch { left, right }) => {
                    let bit = br.get_bits(1);
                    idx = if bit == 0 { *left } else { *right };
                    if idx == NO_CHILD {
                        // SQZ.EXE: unused prefix decodes to 0
                        // The bit has already been consumed
                        return 0;
                    }
                }
                None => return 0,  // Safety fallback
            }
        }
    }
}

// =====================================================================
// Read Huffman lengths (PT table)
// =====================================================================

fn read_pt_len(br: &mut BitReader, nn: usize, nbit: u8, special: i32) -> Vec<u8> {
    let n = br.get_bits(nbit) as usize;
    if n == 0 {
        let c = br.get_bits(nbit) as usize;
        let mut pt_len = vec![0u8; nn];
        if c < nn {
            pt_len[c] = 1; // Mark single symbol
        }
        return pt_len;
    }

    let mut pt_len = vec![0u8; nn];
    let mut i = 0usize;
    while i < n.min(nn) {
        let mut c = br.peek_bits(3);
        if c == 7 {
            br.skip_bits(3);
            while br.get_bits(1) == 1 {
                c += 1;
            }
        } else {
            br.skip_bits(3);
        }
        pt_len[i] = c as u8;
        i += 1;

        if i == special as usize {
            let zeros = br.get_bits(2) as usize + 1;  // SQZ uses (value + 1) zeros
            for _ in 0..zeros {
                if i < n.min(nn) {
                    pt_len[i] = 0;
                    i += 1;
                }
            }
        }
    }

    pt_len
}

// =====================================================================
// Read C lengths using PT table
// =====================================================================

fn read_c_len(br: &mut BitReader, pt_tree: &HuffTree) -> Vec<u8> {
    let n = br.get_bits(CBIT) as usize;
    eprintln!("    read_c_len: n = {}", n);
    if n == 0 {
        let c = br.get_bits(CBIT) as usize;
        eprintln!("    read_c_len: constant c = {}", c);
        let mut c_len = vec![0u8; NC];
        if c < NC {
            c_len[c] = 1;
        }
        return c_len;
    }

    let mut c_len = vec![0u8; NC];
    let mut i = 0usize;
    let mut debug_count = 0;
    while i < n.min(NC) {
        let c = pt_tree.decode(br);
        if debug_count < 20 {
            eprint!("      sym={} at i={}", c, i);
        }
        if c <= 2 {
            // SQZ-specific RLE:
            // sym=0: 2 zeros
            // sym=1: 4-19 zeros (4 bits extra)
            // sym=2: 0x14 + 7-bit segments (until segment != 0x7f) + 1 zeros
            let count = match c {
                0 => 2usize,
                1 => (br.get_bits(4) as usize) + 4,
                2 => {
                    let mut total = 0x14usize;
                    loop {
                        let add = br.get_bits(7) as usize;
                        total += add;
                        if add != 0x7f {
                            break;
                        }
                    }
                    total + 1
                }
                _ => 1,
            };
            if debug_count < 20 {
                eprintln!(" -> {} zeros", count);
            }
            for _ in 0..count {
                if i < n.min(NC) {
                    c_len[i] = 0;
                    i += 1;
                }
            }
        } else {
            if debug_count < 20 {
                eprintln!(" -> len {}", c - 2);
            }
            c_len[i] = (c - 2) as u8;
            i += 1;
        }
        debug_count += 1;
    }
    
    let non_zero_count = c_len.iter().filter(|&&x| x > 0).count();
    let non_zero_positions: Vec<_> = c_len.iter().enumerate().filter(|(_, &x)| x > 0).take(20).map(|(i, &x)| (i, x)).collect();
    eprintln!("    read_c_len finished: i={}, non_zero_count={}, positions={:?}", i, non_zero_count, non_zero_positions);
    
    // Check specific ASCII chars
    let space_len = c_len.get(32).copied().unwrap_or(0);
    let a_upper_len = c_len.get(65).copied().unwrap_or(0);
    let a_lower_len = c_len.get(97).copied().unwrap_or(0);
    let newline_len = c_len.get(10).copied().unwrap_or(0);
    eprintln!("    ASCII lens: space(32)={}, A(65)={}, a(97)={}, newline(10)={}", 
              space_len, a_upper_len, a_lower_len, newline_len);

    c_len
}

// =====================================================================
// SQZ Method 4 decoder
// =====================================================================

fn unsqz_method4_internal(data: &[u8], original_size: usize) -> Result<Vec<u8>> {
    const WINDOW_SIZE: usize = 0x8000; // 32 KB
    const WINDOW_MASK: usize = WINDOW_SIZE - 1;

    let mut br = BitReader::new(data);
    let mut out = Vec::with_capacity(original_size);
    let mut window = vec![0u8; WINDOW_SIZE];
    let mut win_pos = 0usize;

    let mut block_size = 0i32;
    let mut c_tree: Option<HuffTree> = None;
    let mut p_tree: Option<HuffTree> = None;

    eprintln!("SQZ method4: data len={}, original_size={}", data.len(), original_size);

    let mut symbols_in_block = 0;
    while out.len() < original_size {
        if block_size == 0 {
            if symbols_in_block > 0 {
                eprintln!("  [end of block] symbols_in_block={}, out.len()={}", symbols_in_block, out.len());
            }
            symbols_in_block = 0;
            block_size = br.get_bits(14) as i32;  // 14 bits for block size
            eprintln!("  block_size (14 bits) = {}", block_size);
            if block_size <= 0 {
                break;
            }

            // Read PT table for C lengths
            let pt_len = read_pt_len(&mut br, NT, TBIT, 3);
            eprintln!("  pt_len = {:?}", &pt_len[..NT]);
            let pt_tree = match HuffTree::from_bit_lengths(&pt_len) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("  pt_tree build failed: {}", e);
                    return Err(e);
                }
            };

            // Read C table
            let c_len = read_c_len(&mut br, &pt_tree);
            eprintln!("  c_len (first 20): {:?}", &c_len[..20.min(c_len.len())]);
            c_tree = Some(match HuffTree::from_bit_lengths(&c_len) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("  c_tree build failed: {}", e);
                    return Err(e);
                }
            });

            // Read P table
            let p_len = read_pt_len(&mut br, NP, PBIT, -1);
            eprintln!("  p_len = {:?}", &p_len[..NP]);
            p_tree = Some(match HuffTree::from_bit_lengths(&p_len) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("  p_tree build failed: {}", e);
                    return Err(e);
                }
            });
        }
        block_size -= 1;
        symbols_in_block += 1;

        let c_tree_ref = c_tree.as_ref().unwrap();
        let p_tree_ref = p_tree.as_ref().unwrap();
        
        let c = c_tree_ref.decode(&mut br);
        if out.len() < 30 {
            eprintln!("  decode[{}]: c={} (0x{:x})", out.len(), c, c);
        }
        let _old_out_len = out.len();
        if c < 256 {
            // Literal byte
            let b = c as u8;
            out.push(b);
            window[win_pos] = b;
            win_pos = (win_pos + 1) & WINDOW_MASK;
        } else {
            // Match: length + distance
            let length = (c - 253) as usize; // c - 256 + 3

            let p = p_tree_ref.decode(&mut br) as usize;
            let dist = if p == 0 {
                0usize
            } else {
                let extra = (p - 1) as u8;
                let base = 1usize << extra;
                let add = br.get_bits(extra) as usize;
                base + add
            };

            if out.len() < 50 {
                eprintln!("  sym c={}, length={}, p={}, dist={}", c, length, p, dist);
            }

            let mut src = (win_pos.wrapping_sub(dist + 1)) & WINDOW_MASK;
            for _ in 0..length {
                if out.len() >= original_size {
                    break;
                }
                let b = window[src];
                out.push(b);
                window[win_pos] = b;
                win_pos = (win_pos + 1) & WINDOW_MASK;
                src = (src + 1) & WINDOW_MASK;
            }
        }
    }

    Ok(out)
}

fn accept_candidate(mut out: Vec<u8>, original_size: usize, expected_crc32: u32) -> Option<Vec<u8>> {
    if out.len() < original_size {
        return None;
    }
    out.truncate(original_size);
    if crc32fast::hash(&out) == expected_crc32 {
        Some(out)
    } else {
        None
    }
}

/// Decompress SQZ compressed data.
///
/// Currently only method 4 (the default compression in SQZ.EXE) has a partial
/// implementation. Methods 1-3 are not yet reverse-engineered.
pub fn unsqz(buf: &[u8], original_size: usize, method: u8, expected_crc32: u32) -> Result<Vec<u8>> {
    let ignore_crc = std::env::var("UNARC_SQZ_IGNORE_CRC")
        .map(|v| {
            let v = v.to_ascii_lowercase();
            v == "1" || v == "true" || v == "yes" || v == "on"
        })
        .unwrap_or(false);

    match method {
        0 => {
            // Stored (should not reach here, handled by caller)
            Ok(buf.to_vec())
        }
        1 | 2 | 3 | 4 => {
            if ignore_crc {
                super::unsqz_old::unsqz_sqz_only_raw(buf, original_size, method)
            } else {
                super::unsqz_old::unsqz_sqzexe_only(buf, original_size, method, expected_crc32)
            }
        }
        _ => Err(ArchiveError::unsupported_method(
            "SQZ",
            format!("Unknown(method={})", method),
        )),
    }
}
