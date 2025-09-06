//! HYP (Hyper) Archive Decompressor
//!
//! Ported line-by-line from UNPACK.ASM and COMM.ASM
//! Key insight: ASM uses BYTE offsets consistently (DW = 2 bytes)
//! All indices in this implementation use BYTE offsets, then converted to word indices

use std::io;

// Constants from DEFS.INC - in BYTE units where applicable
const STR_IND_BUF_LEN: usize = 8191; // Number of entries (DW)
const MAX_REC_FREQUENCY: usize = 4096;
const MAX_FREQ: usize = 2;
const MINDIFF: i16 = -4;
const MAXDIFF: i16 = 8;
const DIFF_OFFSET: i16 = 1;
const LSEQUENCE_KEY: usize = 0;
const HUFF_SIZE: usize = 3200;
const MAXMCOUNT: usize = 1350;

// maxlocal bounds (in words, ASM has 2 and 80 in bytes = 1 and 40 words)
const MAXLOCAL_40: usize = 2; // ASM: 2 bytes
const MAXLOCAL_41: usize = 80; // ASM: 80 bytes

/// BitReader - exact port of rBit/rBits macros
/// LSB-first bit reading
struct BitReader<'a> {
    buffer: &'a [u8],
    byte_pos: usize, // bbytepos - current byte position
    bit_cnt: u8,     // bitpos low byte - remaining bits in puffer_byte
    puffer_byte: u8, // bitpos high byte - current byte being read
}

impl<'a> BitReader<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        // ASM: Mov bh,[bitpuffer]; Mov bl,8
        let puffer_byte = if !buffer.is_empty() { buffer[0] } else { 0 };
        Self {
            buffer,
            byte_pos: 0,
            bit_cnt: 8,
            puffer_byte,
        }
    }

    /// rBit macro - read single bit, return in carry (as bool)
    /// Shr BitBuf,1 ; Dec BitCnt ; ...
    #[inline]
    fn read_bit(&mut self) -> bool {
        // Shr BitBuf,1 - shift right, carry = LSB
        let bit = (self.puffer_byte & 1) != 0;
        self.puffer_byte >>= 1;

        // Dec BitCnt
        self.bit_cnt -= 1;
        if self.bit_cnt == 0 {
            // Need to load next byte
            self.byte_pos += 1;
            if self.byte_pos < self.buffer.len() {
                self.puffer_byte = self.buffer[self.byte_pos];
            } else {
                self.puffer_byte = 0;
            }
            self.bit_cnt = 8;
        }

        bit
    }

    /// rBits macro - read CX bits, return in AX
    /// Bits are read LSB first
    fn read_bits(&mut self, count: usize) -> u16 {
        let mut result: u16 = 0;
        let mut mask: u16 = 1;

        for _ in 0..count {
            if self.read_bit() {
                result |= mask;
            }
            mask <<= 1;
        }

        result
    }
}

/// HuffmanState - all arrays use BYTE indices internally
/// Converted to word indices for array access
struct HuffmanState {
    // Huffman tree (indexed by byte offset / 2)
    vater: Vec<u16>,    // DW 2*huffsize+1
    sohn: Vec<u16>,     // DW 2*huffsize+1
    the_freq: Vec<u16>, // DW 2*huffsize+1

    // Index/value/frequency tables
    nindex: Vec<u16>,     // DW StrIndBufLen+1
    nvalue: Vec<u16>,     // DW tabsize+1
    frequencys: Vec<u16>, // DW tabsize+1

    // Frequency arrays
    nfreq: Vec<u16>,    // DW maxfreq+2 - indexed by byte offset
    nfreqmax: Vec<u16>, // DW maxrecfrequency

    // String index buffer
    str_ind_buf: Vec<u16>, // DW StrIndBufLen - indexed by byte offset / 2
    new_index: Vec<u16>,   // DW StrIndBufLen

    // State variables (all in BYTE offsets as in ASM)
    huff_max: u16,          // BYTE offset
    huff_max_index: u16,    // BYTE offset
    teststrings_index: u16, // BYTE offset (tsi * 2)
    low_tsi: u16,           // BYTE offset
    lastposition: u16,      // BYTE offset
    maxlocal: u16,          // BYTE offset
    maxlocal255: u16,       // BYTE offset (maxlocal + 2*255)
    local_offset: u16,      // BYTE offset
    pos_offset: u16,        // BYTE offset
    char_offset: u16,       // BYTE offset
}

impl HuffmanState {
    fn new() -> Self {
        // str_ind_buf is NOT initialized for indices 0-255 in the ASM code.
        // These slots are reserved for single characters. When decode_data
        // encounters ax < 256, it outputs ax directly as a character.
        // str_ind_buf[i] is only used for i >= 256 (filled by ReadSmart).
        let str_ind_buf = vec![0u16; STR_IND_BUF_LEN + 1];

        Self {
            vater: vec![0; 2 * HUFF_SIZE + 2],
            sohn: vec![0; 2 * HUFF_SIZE + 2],
            the_freq: vec![0; 2 * HUFF_SIZE + 2],
            nindex: vec![0; STR_IND_BUF_LEN + 2],
            nvalue: vec![0; STR_IND_BUF_LEN + 400],
            frequencys: vec![0; STR_IND_BUF_LEN + 400],
            nfreq: vec![0; MAX_FREQ + 4],
            nfreqmax: vec![0; MAX_REC_FREQUENCY + 1],
            str_ind_buf,
            new_index: vec![0; STR_IND_BUF_LEN + 1],
            huff_max: 0,
            huff_max_index: 0,
            teststrings_index: 255, // Word value, not byte offset initially
            low_tsi: 2 * 255,       // BYTE offset
            lastposition: 2 * 256,  // BYTE offset
            maxlocal: 0,
            maxlocal255: 0,
            local_offset: 0,
            pos_offset: 0,
            char_offset: 0,
        }
    }

    /// set_vars - calculate offsets based on teststrings_index
    /// All results in BYTE offsets
    fn set_vars(&mut self) {
        // ASM:
        // Mov ax,2*(maxlocal_41-maxlocal_40)
        // Mov Cx,[teststrings_index]
        // Sub Cx,255
        // IMul Cx
        // Mov Cx,StrIndBufLen-255
        // IDiv Cx
        // And Ax,0FFFEh
        // Add Ax,2*maxlocal_40

        let ax = 2 * (MAXLOCAL_41 - MAXLOCAL_40) as i32; // 2*(80-2) = 156
        let cx = self.teststrings_index as i32 - 255;
        let product = ax * cx;
        let divisor = (STR_IND_BUF_LEN - 255) as i32;

        let maxlocal = if divisor > 0 {
            let mut result = product / divisor;
            result &= !1; // And Ax,0FFFEh - make even
            (result + 2 * MAXLOCAL_40 as i32) as u16 // Add 2*maxlocal_40
        } else {
            (2 * MAXLOCAL_40) as u16
        };

        self.maxlocal = maxlocal;
        self.maxlocal255 = maxlocal + 2 * 255; // Add Ax,2*255

        // local_offset = 2*(diff_offset+1) + (maxdiff-mindiff)
        // ASM: mov AX,2*(diff_offset+1)+maxdiff-mindiff
        self.local_offset = (2 * (DIFF_OFFSET + 1) + (MAXDIFF - MINDIFF)) as u16;

        // pos_offset = local_offset + (maxlocal+2)
        // ASM: Add AX,[maxlocal]; Add AX,2
        self.pos_offset = self.local_offset + self.maxlocal + 2;

        // char_offset = pos_offset + 2*(maxfreq+1)
        // ASM: Add AX,2*maxfreq+2
        self.char_offset = self.pos_offset + 2 * MAX_FREQ as u16 + 2;
    }

    /// InitHuffTables - initialize Huffman tables
    fn init_huff_tables(&mut self) {
        // fillword <offset nfreqmax>,0,maxrecfrequency
        for i in 0..MAX_REC_FREQUENCY {
            self.nfreqmax[i] = 0;
        }

        // Initialize nindex, nvalue, frequencys
        // ASM:
        // mov cx,StrIndBufLen
        // Xor Si,Si
        // mov di,[char_offset]
        // @@for1: mov [nindex+si],di
        //         mov [nvalue+di],si
        //         mov [frequencys+di],0
        //         Add di,2
        //         Add si,2
        //         loop @@for1

        let mut di = self.char_offset;
        for si in 0..STR_IND_BUF_LEN {
            let si_byte = (si * 2) as u16;
            let di_word = (di / 2) as usize;
            if di_word < self.nvalue.len() && si < self.nindex.len() {
                self.nindex[si] = di;
                self.nvalue[di_word] = si_byte;
                self.frequencys[di_word] = 0;
            }
            di += 2;
        }

        // fillword <offset nfreq>,<[char_offset]>,maxfreq+2
        for i in 0..=MAX_FREQ + 1 {
            self.nfreq[i] = self.char_offset;
        }

        // Initialize root node
        // mov [Shared.Sohn],1
        // mov [Shared.TheFreq],2
        // mov [Shared.Vater],0
        self.sohn[0] = 1;
        self.the_freq[0] = 2;
        self.vater[0] = 0;

        // mov [HuffMaxindex],2
        // mov [frequencys+0],0
        // mov [HuffMax],2
        self.huff_max_index = 2;
        self.frequencys[0] = 0;
        self.huff_max = 2;

        // mov cx,[char_offset]
        // shr cx,1
        // dec cx
        // @@for: Call ninsert; Loop @@for
        let count = (self.char_offset / 2).saturating_sub(1);
        for _ in 0..count {
            self.ninsert();
        }
    }

    /// ninsert - insert new Huffman node
    fn ninsert(&mut self) {
        let di = self.huff_max as usize;
        let bx = di.saturating_sub(2); // di - 2 (1 word = 2 bytes)
        let di_idx = di / 2;
        let bx_idx = bx / 2;

        if di_idx + 1 >= self.vater.len() {
            return;
        }

        // mov [di+Shared.Vater],bx
        // mov [di+2+Shared.Vater],bx
        self.vater[di_idx] = bx as u16;
        self.vater[di_idx + 1] = bx as u16;

        // mov [di+2+Shared.TheFreq],2
        self.the_freq[di_idx + 1] = 2;

        // mov bx,[HuffMaxindex]
        // inc bx
        // mov [di+2+Shared.Sohn],bx  (HuffMaxindex | 1)
        self.sohn[di_idx + 1] = self.huff_max_index | 1;

        // mov bx,di
        // xchg bx,[di-2+Shared.Sohn]  ; HuffMax --> Sohn[HuffMax-1] --> BX
        // mov [di+Shared.Sohn],bx     ; Sohn[HuffMax]:=Sohn[HuffMax-1]
        let old_child = self.sohn[bx_idx];
        self.sohn[bx_idx] = di as u16;
        self.sohn[di_idx] = old_child;

        // mov [frequencys+bx-1],di  ; frequencys is byte-indexed
        // Note: bx-1 means we treat old_child-1 as byte offset for frequencys
        if old_child > 0 {
            let freq_idx = ((old_child - 1) / 2) as usize;
            if freq_idx < self.frequencys.len() {
                self.frequencys[freq_idx] = di as u16;
            }
        }

        // mov bx,[di-2+Shared.TheFreq]
        // mov [di+Shared.TheFreq],bx
        self.the_freq[di_idx] = self.the_freq[bx_idx];

        // mov bx,[HuffMaxindex]
        // mov [frequencys+bx],di+2 ; frequencys[HuffMaxindex]:= HuffMax+1
        let hmi_idx = (self.huff_max_index / 2) as usize;
        if hmi_idx < self.frequencys.len() {
            self.frequencys[hmi_idx] = (di + 2) as u16;
        }

        // Add bx,2; mov [HuffMaxindex],bx
        self.huff_max_index += 2;

        // Add di,2; mov [HuffMax],di
        self.huff_max = (di + 4) as u16;

        // Sub di,3*2; Jmp ientry (inc_frequency at HuffMax-3)
        if di >= 4 {
            let entry = (di - 4) / 2;
            self.inc_frequency_ientry(entry);
        }
    }

    /// inc_frequency - increase frequency of Huffman node
    /// di is word index into Huffman tree
    fn inc_frequency(&mut self, di: usize) {
        if di >= self.the_freq.len() {
            return;
        }

        let freq = self.the_freq[di] as usize;
        if freq == 0 || freq >= 2 * MAX_REC_FREQUENCY {
            return;
        }

        // mov si,[nfreqmax+bx-2]  (bx = freq, so index is (freq-2)/2 = freq/2-1)
        let freq_idx = (freq / 2).saturating_sub(1);
        if freq_idx >= self.nfreqmax.len() {
            return;
        }

        let mut si = (self.nfreqmax[freq_idx] / 2) as usize;
        // Add [nfreqmax+bx-2],2
        self.nfreqmax[freq_idx] += 2;

        if di != si && di < self.sohn.len() && si < self.sohn.len() {
            // Swap sons
            let sohn_di = self.sohn[di];
            // mov [bx-1+frequencys],si - update frequency pointer
            if sohn_di > 0 && (sohn_di & 1) == 0 {
                // Not a leaf
                self.vater[(sohn_di / 2) as usize] = si as u16;
                self.vater[(sohn_di / 2 + 1) as usize] = si as u16;
            } else if sohn_di > 0 {
                // Leaf
                let freq_ptr = ((sohn_di - 1) / 2) as usize;
                if freq_ptr < self.frequencys.len() {
                    self.frequencys[freq_ptr] = (si * 2) as u16;
                }
            }

            // xchg bx,[si+Shared.Sohn]
            let tmp = self.sohn[si];
            self.sohn[si] = self.sohn[di];
            self.sohn[di] = tmp;

            // Update vater for swapped child
            if tmp > 0 && (tmp & 1) == 0 {
                self.vater[(tmp / 2) as usize] = di as u16;
                self.vater[(tmp / 2 + 1) as usize] = di as u16;
            } else if tmp > 0 {
                let freq_ptr = ((tmp - 1) / 2) as usize;
                if freq_ptr < self.frequencys.len() {
                    self.frequencys[freq_ptr] = (di * 2) as u16;
                }
            }
        }

        // Add [si+Shared.TheFreq],2
        if si < self.the_freq.len() {
            self.the_freq[si] += 2;
        }

        // mov di,[si+Shared.Vater]; i:=Vater[j]
        if si < self.vater.len() {
            let parent = (self.vater[si] / 2) as usize;
            if parent > 0 {
                self.inc_frequency_ientry(parent);
            }
        }
    }

    /// inc_frequency from ientry point (skips initial setup)
    fn inc_frequency_ientry(&mut self, mut di: usize) {
        let mut iterations = 0;
        loop {
            iterations += 1;
            if iterations > 10000 || di >= self.the_freq.len() {
                return;
            }

            let freq = self.the_freq[di] as usize;
            if freq == 0 || freq >= 2 * MAX_REC_FREQUENCY {
                return;
            }

            let freq_idx = (freq / 2).saturating_sub(1);
            if freq_idx >= self.nfreqmax.len() {
                return;
            }

            let si = (self.nfreqmax[freq_idx] / 2) as usize;
            self.nfreqmax[freq_idx] += 2;

            if di != si && di < self.sohn.len() && si < self.sohn.len() {
                // Swap logic
                let sohn_di = self.sohn[di];
                if sohn_di > 0 && (sohn_di & 1) == 0 {
                    if (sohn_di / 2) as usize + 1 < self.vater.len() {
                        self.vater[(sohn_di / 2) as usize] = (si * 2) as u16;
                        self.vater[(sohn_di / 2 + 1) as usize] = (si * 2) as u16;
                    }
                } else if sohn_di > 0 {
                    let freq_ptr = ((sohn_di - 1) / 2) as usize;
                    if freq_ptr < self.frequencys.len() {
                        self.frequencys[freq_ptr] = (si * 2) as u16;
                    }
                }

                let tmp = self.sohn[si];
                self.sohn[si] = self.sohn[di];
                self.sohn[di] = tmp;

                if tmp > 0 && (tmp & 1) == 0 {
                    if (tmp / 2) as usize + 1 < self.vater.len() {
                        self.vater[(tmp / 2) as usize] = (di * 2) as u16;
                        self.vater[(tmp / 2 + 1) as usize] = (di * 2) as u16;
                    }
                } else if tmp > 0 {
                    let freq_ptr = ((tmp - 1) / 2) as usize;
                    if freq_ptr < self.frequencys.len() {
                        self.frequencys[freq_ptr] = (di * 2) as u16;
                    }
                }
            }

            if si < self.the_freq.len() {
                self.the_freq[si] += 2;
            }

            if si < self.vater.len() {
                let parent = self.vater[si] as usize;
                if parent == 0 {
                    return;
                }
                di = parent / 2;
            } else {
                return;
            }
        }
    }

    /// inc_posfreq - increment position frequency (called from tab_decode)
    fn inc_posfreq(&mut self, si: usize) {
        // si is byte offset into nvalue table
        let si_idx = si / 2;
        if si_idx >= self.frequencys.len() {
            return;
        }

        // mov bx,[frequencys+si]
        let bx = self.frequencys[si_idx] as usize;
        let bx_idx = (bx / 2) + 1;
        if bx_idx >= self.nfreq.len() {
            return;
        }

        // mov di,[nfreq+bx+2]
        let di = self.nfreq[bx_idx] as usize;
        let di_idx = di / 2;

        // mov bx,[nvalue+si]
        let nval_bx = if si_idx < self.nvalue.len() {
            self.nvalue[si_idx] as usize
        } else {
            return;
        };

        // mov [nindex+bx],di
        let nval_idx = nval_bx / 2;
        if nval_idx < self.nindex.len() {
            self.nindex[nval_idx] = di as u16;
        }

        // xchg bx,[nvalue+di]
        if di_idx < self.nvalue.len() {
            let old_nvalue_di = self.nvalue[di_idx];
            self.nvalue[di_idx] = nval_bx as u16;

            // mov [nindex+bx],si
            let old_idx = (old_nvalue_di / 2) as usize;
            if old_idx < self.nindex.len() {
                self.nindex[old_idx] = si as u16;
            }

            // mov [nvalue+si],bx
            if si_idx < self.nvalue.len() {
                self.nvalue[si_idx] = old_nvalue_di;
            }
        }

        // mov bx,[frequencys+di]
        if di_idx >= self.frequencys.len() {
            return;
        }
        let freq_di = self.frequencys[di_idx] as usize;

        if freq_di >= 2 * MAX_FREQ as usize {
            // @@ins_huff: Call ninsert
            self.ninsert();
            let new_di = (self.huff_max.saturating_sub(4) / 2) as usize;
            if new_di > 0 {
                self.inc_frequency(new_di);
            }
        } else {
            // inc(f^.frequenz)
            if di_idx < self.frequencys.len() {
                self.frequencys[di_idx] = (freq_di + 2) as u16;
            }
            // inc(freq[SI^.frequenz])
            let freq_idx = (freq_di / 2) + 1;
            if freq_idx < self.nfreq.len() {
                self.nfreq[freq_idx] = (di + 2) as u16;
            }
        }
    }

    /// DecodeHuffEntry - decode one Huffman symbol
    /// Returns byte offset into symbol table
    fn decode_huff_entry(&mut self, reader: &mut BitReader) -> u16 {
        // Xor Si,Si ; i:=0
        let mut si: usize = 0;
        let mut iterations = 0;
        let mut path: Vec<(usize, u16, bool, bool)> = Vec::new(); // (si, child, bit, is_leaf)

        loop {
            iterations += 1;
            if iterations > 10000 {
                eprintln!("ERROR: decode_huff_entry infinite loop at si={}", si);
                return 0;
            }

            if si >= self.sohn.len() {
                eprintln!("ERROR: si {} out of bounds", si);
                return 0;
            }

            // mov si,[si+Shared.Sohn] ; Si:= Sohn(Si)
            let child = self.sohn[si];
            let is_leaf = (child & 1) != 0;

            // test si,1; jne @@ende
            if is_leaf {
                // Leaf node
                // dec si; mov ax,si
                let ax = child - 1;

                path.push((si, child, false, true));
                if path.len() <= 20 {
                    eprintln!("    decode_huff_entry: path={:?} -> leaf ax={}", path, ax);
                }

                // mov di,[frequencys+si] ; di = frequencys[(ax)/2]
                let freq_idx = (ax / 2) as usize;
                if freq_idx < self.frequencys.len() {
                    let di = (self.frequencys[freq_idx] / 2) as usize;
                    // Jmp inc_frequency
                    self.inc_frequency(di);
                }

                return ax;
            }

            // rBit; Jnc @@loop; Add Si,2
            let bit = reader.read_bit();
            path.push((si, child, bit, false));
            si = (child / 2) as usize;
            if bit {
                si += 1;
            }
        }
    }

    /// TabDecode macro - decode from frequency table
    /// bx is byte offset parameter
    fn tab_decode(&mut self, reader: &mut BitReader, bx: u16) -> u16 {
        // Sub Bx,[pos_offset]
        let bx_adj = bx.wrapping_sub(self.pos_offset);
        let bx_idx = (bx_adj / 2) as usize;

        // Mov Ax,[nfreq+Bx]
        // Mov Si,[nfreq+Bx+2]
        let ax = if bx_idx < self.nfreq.len() {
            self.nfreq[bx_idx] as i32
        } else {
            0
        };
        let si = if bx_idx + 1 < self.nfreq.len() {
            self.nfreq[bx_idx + 1] as i32
        } else {
            0
        };

        // Sub Ax,Si; Shr Ax,1
        let dx = ((ax - si) / 2) as u16;

        // Mov Cx,1; Mov Ax,Cx
        let mut cx: u16 = 1;
        let mut ax: u16 = 1;

        // @@repeat: rBit; jc @@skp; Xor Ax,Cx
        // @@skp: shl Cx,1; Or Ax,Cx; Cmp Ax,Dx; jbe @@repeat
        loop {
            if !reader.read_bit() {
                ax ^= cx;
            }
            cx <<= 1;
            ax |= cx;
            if ax > dx {
                break;
            }
        }

        // Xor Ax,Cx
        ax ^= cx;

        // Shl Ax,1; Add si,ax
        let result_idx = (si as u16 + ax * 2) as usize;

        // Mov ax,[nvalue+si]
        let result = if result_idx / 2 < self.nvalue.len() {
            self.nvalue[result_idx / 2]
        } else {
            0
        };

        // Call inc_posfreq
        self.inc_posfreq(result_idx);

        result
    }

    /// ClearWhenFull - compress str_ind_buf when full
    fn clear_when_full(&mut self) {
        // Mov [Low_tsi],2*255
        self.low_tsi = 2 * 255;

        // Cmp [teststrings_index],StrIndBufLen; jne @@exit
        if self.teststrings_index != (STR_IND_BUF_LEN * 2) as u16 {
            return;
        }

        // fillword <Offset Shared.NewIndex>,0,StrIndBufLen
        for i in 0..STR_IND_BUF_LEN {
            self.new_index[i] = 0;
        }

        // Xor Cx,Cx
        let mut cx: usize = 0;

        // Mov Di,2*StrIndBufLen
        // @@for: Sub Di,2; Call markiere; Cmp Cx,maxmcount; jb @@fOr
        let mut di = 2 * STR_IND_BUF_LEN;
        loop {
            di -= 2;
            self.markiere(di, &mut cx);
            if cx >= MAXMCOUNT || di == 0 {
                break;
            }
        }

        // Add Cx,255; shl Cx,1; Mov [Low_tsi],Cx
        cx += 255;
        self.low_tsi = (cx * 2) as u16;

        // Mov Bx,2*255; Mov Di,2*254
        let mut bx: usize = 2 * 255;
        let mut di: usize = 2 * 254;

        // @@for2: Add Di,2
        loop {
            di += 2;
            let di_idx = di / 2;
            if di_idx >= self.new_index.len() {
                break;
            }

            // Cmp [Di+Shared.NewIndex],0; je @@for2
            if self.new_index[di_idx] == 0 {
                continue;
            }

            // Mov [Di+Shared.NewIndex],Bx
            self.new_index[di_idx] = bx as u16;

            // Mov Si,[StrIndBuf+Di]
            let mut si = if di_idx < self.str_ind_buf.len() {
                self.str_ind_buf[di_idx] as usize
            } else {
                continue;
            };

            // Cmp Si,2*256; jb @@1
            if si >= 2 * 256 {
                // Mov Si,[Si+Shared.NewIndex]
                let si_idx = si / 2;
                if si_idx < self.new_index.len() {
                    si = self.new_index[si_idx] as usize;
                }
            }

            // Mov [StrIndBuf+Bx],Si
            let bx_idx = bx / 2;
            if bx_idx < self.str_ind_buf.len() {
                self.str_ind_buf[bx_idx] = si as u16;
            }

            // Add Bx,2
            bx += 2;

            // Cmp Bx,[Low_tsi]; jb @@for2
            if bx >= self.low_tsi as usize {
                break;
            }
        }
    }

    /// markiere - mark entries for preservation
    /// di is byte offset
    fn markiere(&mut self, di: usize, cx: &mut usize) {
        let di_idx = di / 2;
        if di_idx >= self.new_index.len() {
            return;
        }

        // Cmp [Di+Shared.NewIndex],0; Jne @@exit
        if self.new_index[di_idx] != 0 {
            return;
        }

        // Mov Si,[StrIndBuf+Di]
        let si = if di_idx < self.str_ind_buf.len() {
            self.str_ind_buf[di_idx] as usize
        } else {
            return;
        };

        // Cmp Si,2*256; jb @@no_recurse
        if si >= 2 * 256 {
            // Push Di Si; Mov Di,Si; Sub Di,2; Call markiere
            let new_di = si - 2;
            self.markiere(new_di, cx);

            // Pop Si Di; Cmp Di,Si; je @@no_recurse
            if di != si {
                // Push Di; Mov Di,Si; Call markiere; Pop Di
                self.markiere(si, cx);
            }
        }

        // Mov [Di+Shared.NewIndex],marked (1)
        self.new_index[di_idx] = 1;

        // Inc Cx
        *cx += 1;
    }
}

/// Main decompression function
pub(crate) fn unpack_hyp(compressed_buffer: &[u8], original_size: usize) -> io::Result<Vec<u8>> {
    let mut output = Vec::with_capacity(original_size);
    let mut reader = BitReader::new(compressed_buffer);
    let mut state = HuffmanState::new();

    let mut iteration = 0;
    // Main decode loop from Decode procedure
    loop {
        iteration += 1;
        eprintln!(
            "=== Iteration {}: tsi={}, low_tsi={}",
            iteration, state.teststrings_index, state.low_tsi
        );

        state.clear_when_full();
        read_smart(&mut state, &mut reader)?;
        decode_data(&mut state, &mut output);

        eprintln!(
            "After decode: output.len={}, tsi={}",
            output.len(),
            state.teststrings_index
        );

        // shr [teststrings_index],1 ; convert back to word count
        // Cmp [teststrings_index],255; jne @@repeat
        if state.teststrings_index / 2 == 255 {
            break;
        }

        if iteration > 100 {
            eprintln!("ERROR: Too many iterations!");
            break;
        }
    }

    Ok(output)
}

/// ReadSmart - read and decode compressed data block
fn read_smart(state: &mut HuffmanState, reader: &mut BitReader) -> io::Result<()> {
    // Mov Cx,13; rBits
    // mov [teststrings_index],ax
    let tsi = reader.read_bits(13);
    state.teststrings_index = tsi;

    eprintln!("read_smart: tsi from bits = {}", tsi);

    // Call set_vars; Call InitHuffTables
    state.set_vars();
    state.init_huff_tables();

    eprintln!(
        "read_smart: after init, char_offset={}, pos_offset={}, local_offset={}",
        state.char_offset, state.pos_offset, state.local_offset
    );

    // Debug: Print Huffman tree structure
    eprintln!("Huffman tree after init:");
    eprintln!("  sohn[0..5]: {:?}", &state.sohn[0..5]);
    eprintln!("  vater[0..5]: {:?}", &state.vater[0..5]);
    eprintln!("  the_freq[0..5]: {:?}", &state.the_freq[0..5]);
    eprintln!(
        "  huff_max={}, huff_max_index={}",
        state.huff_max, state.huff_max_index
    );
    eprintln!("  nvalue[0..10]: {:?}", &state.nvalue[0..10]);
    eprintln!(
        "  nvalue[30..40] (char_offset/2={}): {:?}",
        state.char_offset / 2,
        &state.nvalue[30..40]
    );
    eprintln!("  frequencys[0..10]: {:?}", &state.frequencys[0..10]);

    // shl [teststrings_index],1
    state.teststrings_index *= 2;

    // mov Di,[Low_tsi]; Sub Di,2
    let mut di = (state.low_tsi - 2) as i32;

    // mov [lastposition],2*256
    state.lastposition = 2 * 256;

    // mov Ax,[char_offset]; Add Ax,2*255; mov [nfreq+0],Ax
    state.nfreq[0] = state.char_offset + 2 * 255;

    eprintln!(
        "read_smart: di_start={}, tsi*2={}",
        di, state.teststrings_index
    );

    let mut loop_count = 0;
    // @@while: Add Di,2
    loop {
        di += 2;
        loop_count += 1;

        // Cmp Di,[teststrings_index]; jae @@endwhile
        if di as u16 >= state.teststrings_index {
            eprintln!(
                "read_smart: exiting loop after {} iterations, di={}",
                loop_count, di
            );
            break;
        }

        if loop_count > 50000 {
            eprintln!("ERROR: read_smart loop too long!");
            break;
        }

        let di_u = di as usize;
        let di_idx = di_u / 2;

        if loop_count <= 10 {
            eprintln!(
                "  read_smart loop {}: di={} di_idx={}",
                loop_count, di, di_idx
            );
        }

        // Cmp Di,[maxlocal255]; jbe @@endcase
        if di as u16 > state.maxlocal255 {
            // mov Ax,Di; Add Ax,[char_offset]; Sub Ax,[maxlocal]
            // mov [nfreq+0],Ax
            let ax = di as u16 + state.char_offset - state.maxlocal;
            state.nfreq[0] = ax;
        }

        // Push Di; Call DecodeHuffEntry; Pop Di
        let ax = state.decode_huff_entry(reader);

        // Mov Bx,AX
        let bx = ax;

        // Debug: show what we're storing
        if loop_count <= 10 {
            eprintln!(
                "    decoded bx={}, local_offset={}, pos_offset={}, char_offset={}",
                bx, state.local_offset, state.pos_offset, state.char_offset
            );
        }

        if bx == 2 * LSEQUENCE_KEY as u16 {
            // dec_lseq: lastposition sequence
            // mov ax,[lastposition]; Add ax,2*2
            let mut ax = state.lastposition + 4;

            // mov [StrIndBuf+Di],ax
            if di_idx < state.str_ind_buf.len() {
                state.str_ind_buf[di_idx] = ax;
                if loop_count <= 10 {
                    eprintln!("    -> dec_lseq: stored {} at idx {}", ax, di_idx);
                }
            }
            di += 2;

            // @@dloop: Add ax,2*2; Mov [StrIndBuf+Di],ax; Add Di,2; rBit; Jc @@dloop
            loop {
                ax += 4;
                let new_di_idx = (di as usize) / 2;
                if new_di_idx < state.str_ind_buf.len() {
                    state.str_ind_buf[new_di_idx] = ax;
                }
                di += 2;
                if !reader.read_bit() {
                    break;
                }
            }

            // Sub Di,2
            di -= 2;
            state.lastposition = ax;
        } else if bx < state.local_offset {
            // dec_diff: Sub Bx,2*diff_offset-mindiff; Add Bx,[lastposition]
            // mindiff = -4, diff_offset = 1
            // 2*diff_offset - mindiff = 2*1 - (-4) = 6
            let diff = bx as i16 - (2 * DIFF_OFFSET - MINDIFF);
            let new_pos = (state.lastposition as i32 + diff as i32) as u16;

            if di_idx < state.str_ind_buf.len() {
                state.str_ind_buf[di_idx] = new_pos;
                if loop_count <= 10 {
                    eprintln!(
                        "    -> dec_diff: bx={} diff={} lastpos={} => stored {} at idx {}",
                        bx, diff, state.lastposition, new_pos, di_idx
                    );
                }
            }
            state.lastposition = new_pos;
        } else if bx < state.pos_offset {
            // dec_local: Sub Bx,[local_offset]; neg Bx; Add Bx,Di
            let offset = bx.wrapping_sub(state.local_offset);
            let neg_offset = (-(offset as i16)) as i32;
            let new_pos = (di + neg_offset) as u16;

            if di_idx < state.str_ind_buf.len() {
                state.str_ind_buf[di_idx] = new_pos;
                if loop_count <= 10 {
                    eprintln!(
                        "    -> dec_local: bx={} offset={} neg={} di={} => stored {} at idx {}",
                        bx, offset, neg_offset, di, new_pos, di_idx
                    );
                }
            }
            state.lastposition = new_pos;
        } else if bx < state.char_offset {
            // dec_table: TabDecode
            let val = state.tab_decode(reader, bx);

            // Cmp ax,2*256; jb @@move_char
            if val >= 2 * 256 {
                // @@move_pos
                if di_idx < state.str_ind_buf.len() {
                    state.str_ind_buf[di_idx] = val;
                }
                state.lastposition = val;
            } else {
                // @@move_char: shr ax,1
                if di_idx < state.str_ind_buf.len() {
                    state.str_ind_buf[di_idx] = val / 2;
                }
            }
        } else {
            // direct value: mov ax,[nvalue+Bx]
            let bx_idx = (bx / 2) as usize;
            let val = if bx_idx < state.nvalue.len() {
                state.nvalue[bx_idx]
            } else {
                0
            };

            // Cmp ax,2*256; jb @@move_char
            if val >= 2 * 256 {
                // @@move_pos
                if di_idx < state.str_ind_buf.len() {
                    state.str_ind_buf[di_idx] = val;
                }
                state.lastposition = val;
            } else {
                // @@move_char: shr ax,1
                if di_idx < state.str_ind_buf.len() {
                    state.str_ind_buf[di_idx] = val / 2;
                }
            }
        }
    }

    // shr [teststrings_index],1 happens in caller's check
    Ok(())
}

/// DecodeData - decode str_ind_buf to output bytes
/// This follows the exact ASM logic from UNPACK.ASM
/// CRITICAL: ASM uses BYTE offsets (bx, ax) when accessing str_ind_buf!
/// If bx/2 < 256, it's a character reference (the "base" 256 characters)
/// If bx/2 >= 256, it's a position in str_ind_buf that was filled by ReadSmart
fn decode_data(state: &mut HuffmanState, output: &mut Vec<u8>) {
    // ASM: Mov Si,[Low_tsi]
    let mut si = state.low_tsi as usize; // BYTE offset
                                         // ASM: Mov Cx,[teststrings_index]; Shl Cx,1
    let tsi_bytes = state.teststrings_index as usize; // Already in bytes (doubled in read_smart)

    if si >= tsi_bytes {
        eprintln!(
            "decode_data: si={} >= tsi_bytes={}, returning",
            si, tsi_bytes
        );
        return;
    }

    eprintln!("decode_data: si={} to tsi_bytes={}", si, tsi_bytes);
    eprintln!(
        "  str_ind_buf[254..260]: {:?}",
        &state.str_ind_buf[254..260]
    );

    // ASM uses the CPU stack for this - we use a Vec
    let mut stack: Vec<u16> = Vec::with_capacity(256);
    let mut entry_count = 0;
    let mut total_output: usize = 0;

    // ASM: @@EnterLoop
    loop {
        if si >= tsi_bytes {
            eprintln!(
                "decode_data: done, processed {} entries, output {} bytes",
                entry_count, total_output
            );
            return;
        }

        entry_count += 1;

        // ASM: Mov Ax,[Si]; Add Si,2
        // Si is a BYTE offset, so [Si] accesses StrIndBuf at word index si/2
        let si_idx = si / 2;
        let mut ax = if si_idx < state.str_ind_buf.len() {
            state.str_ind_buf[si_idx]
        } else {
            return;
        };
        si += 2;

        if entry_count <= 5 {
            eprintln!(
                "  decode_data entry {}: si_byte={} si_idx={} ax={}",
                entry_count,
                si - 2,
                si_idx,
                ax
            );
        }

        // ASM: Or ah,ah; Je @@one_part; Jmp @@two_parts
        // ah = high byte of ax. If ax < 256, ah == 0
        loop {
            if ax < 256 {
                // @@one_part: Mov [Di],Al; Inc Di
                output.push(ax as u8);
                total_output += 1;

                if entry_count <= 5 {
                    eprintln!(
                        "      output char: {} ('{}')",
                        ax,
                        if ax >= 32 && ax < 127 {
                            ax as u8 as char
                        } else {
                            '?'
                        }
                    );
                }

                // ASM: Cmp sp,bp; Jne @@GetStack
                if let Some(popped) = stack.pop() {
                    // @@GetStack: Pop Ax; Or Ah,Ah; Je @@one_part
                    ax = popped;
                    // Continue the inner loop (check if ax < 256 again)
                } else {
                    // Stack is empty, get next entry
                    break;
                }
            } else {
                // @@two_parts: Mov bx,ax
                let bx = ax; // bx is a BYTE offset!
                let bx_idx = (bx / 2) as usize;

                // CRITICAL: If bx_idx < 256, this is a reference to a "base character"
                // These are the first 256 positions that represent single characters.
                // str_ind_buf[0..255] are not explicitly filled - they represent chars 0-255.
                if bx_idx < 256 {
                    // This is a direct character reference: char = bx_idx
                    output.push(bx_idx as u8);
                    total_output += 1;

                    if entry_count <= 5 {
                        eprintln!(
                            "      base char (idx<256): {} ('{}')",
                            bx_idx,
                            if bx_idx >= 32 && bx_idx < 127 {
                                bx_idx as u8 as char
                            } else {
                                '?'
                            }
                        );
                    }

                    // Pop from stack or break
                    if let Some(popped) = stack.pop() {
                        ax = popped;
                    } else {
                        break;
                    }
                    continue;
                }

                // bx_idx >= 256: This is a position filled by ReadSmart
                if bx_idx >= state.str_ind_buf.len() {
                    break;
                }

                // ASM: Mov ax,[StrIndBuf+bx]
                let tail = state.str_ind_buf[bx_idx];

                // ASM: Cmp ax,bx; jne @@direct; Mov ax,[StrIndBuf+bx-2]
                // Self-reference check: if StrIndBuf[bx] == bx, use predecessor instead
                let to_push = if tail == bx {
                    // Self-reference detected, use predecessor at bx-2
                    state.str_ind_buf[bx_idx - 1]
                } else {
                    tail
                };

                // @@direct: push ax (push to_push onto stack)
                stack.push(to_push);

                // ASM: Mov Ax,[StrIndBuf+bx-2]
                // Get predecessor (word at index bx_idx - 1)
                ax = state.str_ind_buf[bx_idx - 1];

                if entry_count <= 5 && stack.len() <= 20 {
                    eprintln!(
                        "      two_parts: bx={} bx_idx={} tail={} pushed={} pred={}",
                        bx, bx_idx, tail, to_push, ax
                    );
                }

                // ASM: Or Ah,Ah; Jne @@two_parts
                // Continue inner loop
            }
        }
    }
}
