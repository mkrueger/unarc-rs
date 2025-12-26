//! Distilled (Method 11) decompression for PAK archives.
//!
//! ## Algorithm Overview
//!
//! Distilled combines LZSS with Huffman coding:
//!
//! 1. **Dynamic Huffman tree**: Stored in the data header, used for literals and length codes.
//! 2. **Static offset table**: Pre-defined Huffman table for offset base values (0-63).
//! 3. **Position-dependent extra bits**: Offset precision increases with output position.
//!
//! ## PAK.EXE Analysis
//!
//! Reverse-engineered from PAK.EXE v2.51 using radare2.
//! - Initialization: `fcn.0001110b` (36 bytes)
//! - Main loop: `fcn.0001112f` (931 bytes)
//! - Dispatch: Method 0x0B at address 0x5e3c
//!
//! ## Data Format
//!
//! ```text
//! Header:
//!   num_nodes: u16 LE    - Number of Huffman tree nodes (2..=0x274)
//!   code_length: u8      - Bits per node value (1..=16)
//!   nodes: [u16; n]      - Tree node array (packed, code_length bits each)
//!
//! Compressed data:
//!   Huffman-coded symbols using the dynamic tree
//! ```
//!
//! ## Symbol Interpretation
//!
//! - 0x00..0xFF: Literal byte
//! - 0x100: EOF marker
//! - 0x101+: Length code (length = symbol - 0x101 + 3)
//!   - Followed by offset symbol from static table
//!   - Followed by extra bits based on output position
//!
//! ## Static Offset Table (from PAK.EXE at 0x11553/0x11593)
//!
//! 64 entries with bit lengths 3-8 and corresponding Huffman codes.
//! See `distilled_table.rs` for the pre-computed lookup table.
//!
//! ## Extra Bits Thresholds (from PAK.EXE)
//!
//! | Position >= | Extra bits |
//! |-------------|------------|
//! | 0x0FC4      | 7          |
//! | 0x07C4      | 6          |
//! | 0x03C4      | 5          |
//! | 0x01C4      | 4          |
//! | 0x00C4      | 3          |
//! | 0x0044      | 2          |
//! | 0x0004      | 1          |
//! | < 4         | 0          |

use super::distilled_table::OFFSET_DECODE_TABLE;
use crate::error::{ArchiveError, Result};
use bitstream_io::{BitRead, BitReader, LittleEndian};

/// EOF marker symbol value.
const EOF_SYMBOL: u16 = 0x100;

/// Maximum valid node count (from PAK.EXE: cmp dx, 0x275).
const MAX_NODES: usize = 0x274;

/// Huffman tree node representation.
#[derive(Clone, Copy, Debug)]
enum HuffmanNode {
    /// Leaf node containing a symbol value.
    Leaf(u16),
    /// Branch node with (left_child, right_child) indices.
    Branch(usize, usize),
}

/// Dynamic Huffman tree built from PAK data header.
struct HuffmanTree {
    /// Tree nodes in traversal order.
    nodes: Vec<HuffmanNode>,
}

impl HuffmanTree {
    /// Build tree from raw node data.
    ///
    /// # Format
    ///
    /// The node array contains indices: for node at position i, the values
    /// at node_data[i*2] and node_data[i*2+1] are the left and right children.
    ///
    /// If child >= num_nodes, it's a leaf with symbol = child - num_nodes.
    /// If child < num_nodes, it's an internal node reference.
    ///
    /// Tree is built starting from root at index (num_nodes - 2).
    fn build(node_data: &[i32], num_nodes: usize) -> Result<Self> {
        let mut tree = Self {
            nodes: Vec::with_capacity(num_nodes * 2),
        };

        if num_nodes >= 2 {
            tree.build_subtree(node_data, num_nodes - 2, num_nodes, 0)?;
        }

        Ok(tree)
    }

    /// Recursively build subtree.
    fn build_subtree(
        &mut self,
        node_data: &[i32],
        node_idx: usize,
        num_nodes: usize,
        depth: usize,
    ) -> Result<usize> {
        // Prevent stack overflow from malformed data.
        if depth > 64 {
            return Err(ArchiveError::decompression_failed(
                "Distilled",
                "tree depth exceeded",
            ));
        }

        let tree_idx = self.nodes.len();

        if node_idx >= num_nodes {
            // Leaf: symbol = node_idx - num_nodes.
            let symbol = (node_idx - num_nodes) as u16;
            self.nodes.push(HuffmanNode::Leaf(symbol));
        } else {
            // Internal node.
            if node_idx + 1 >= node_data.len() {
                return Err(ArchiveError::decompression_failed(
                    "Distilled",
                    format!("invalid node index: {}", node_idx),
                ));
            }

            let left_child = node_data[node_idx] as usize;
            let right_child = node_data[node_idx + 1] as usize;

            // Reserve space for this node.
            self.nodes.push(HuffmanNode::Branch(0, 0));

            // Build children.
            let left_idx = self.build_subtree(node_data, left_child, num_nodes, depth + 1)?;
            let right_idx = self.build_subtree(node_data, right_child, num_nodes, depth + 1)?;

            // Update node with child indices.
            self.nodes[tree_idx] = HuffmanNode::Branch(left_idx, right_idx);
        }

        Ok(tree_idx)
    }

    /// Decode one symbol from the bitstream.
    fn decode<R: BitRead>(&self, reader: &mut R) -> Result<u16> {
        if self.nodes.is_empty() {
            return Err(ArchiveError::decompression_failed(
                "Distilled",
                "empty tree",
            ));
        }

        let mut idx = 0;
        // Safety limit: tree traversal should never exceed tree depth
        // Max depth is bounded by number of nodes
        let max_iterations = self.nodes.len() + 1;
        let mut iterations = 0;

        loop {
            iterations += 1;
            if iterations > max_iterations {
                return Err(ArchiveError::decompression_failed(
                    "Distilled",
                    "tree traversal loop detected (corrupt data or wrong password)",
                ));
            }

            if idx >= self.nodes.len() {
                return Err(ArchiveError::decompression_failed(
                    "Distilled",
                    "invalid tree traversal",
                ));
            }

            match self.nodes[idx] {
                HuffmanNode::Leaf(sym) => return Ok(sym),
                HuffmanNode::Branch(left, right) => {
                    let bit = reader.read_bit().map_err(|e| {
                        ArchiveError::decompression_failed("Distilled", format!("read error: {e}"))
                    })?;
                    // Bit 0 = left, bit 1 = right.
                    idx = if bit { right } else { left };
                }
            }
        }
    }
}

/// Calculate extra bits for offset based on output position.
///
/// These thresholds are from PAK.EXE (0x1000 - 0x3c = 0xFC4, etc.).
#[inline]
fn extra_bits_for_position(pos: usize) -> u32 {
    // Thresholds: 0xFC4, 0x7C4, 0x3C4, 0x1C4, 0xC4, 0x44, 0x04
    if pos >= 0x0FC4 {
        7
    } else if pos >= 0x07C4 {
        6
    } else if pos >= 0x03C4 {
        5
    } else if pos >= 0x01C4 {
        4
    } else if pos >= 0x00C4 {
        3
    } else if pos >= 0x0044 {
        2
    } else if pos >= 0x0004 {
        1
    } else {
        0
    }
}

/// Decode offset symbol using the static Huffman table.
///
/// Reads bits one at a time until a valid symbol is found in the lookup table.
fn decode_offset_symbol<R: BitRead>(reader: &mut R) -> Result<u8> {
    let mut code: u16 = 0;
    let mut len: u8 = 0;

    loop {
        let bit = reader.read_bit().map_err(|e| {
            ArchiveError::decompression_failed("Distilled", format!("read error: {e}"))
        })?;

        // Build code LSB-first (little-endian bit order).
        code |= (bit as u16) << len;
        len += 1;

        // Check lookup table.
        if len <= 8 {
            let mask = (1u16 << len) - 1;
            let idx = (code & mask) as usize;
            let sym = OFFSET_DECODE_TABLE[len as usize][idx];
            if sym >= 0 {
                return Ok(sym as u8);
            }
        }

        if len > 8 {
            return Err(ArchiveError::decompression_failed(
                "Distilled",
                format!("no matching offset code: {code:#x} len={len}"),
            ));
        }
    }
}

/// Decompress Distilled (Method 11) data.
///
/// # Arguments
/// * `input` - Compressed data with header
///
/// # Returns
/// Decompressed data bytes.
pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    if input.len() < 3 {
        return Ok(Vec::new());
    }

    // Parse header.
    let num_nodes = u16::from_le_bytes([input[0], input[1]]) as usize;
    let code_length = input[2];

    // Validate header.
    if !(2..=MAX_NODES).contains(&num_nodes) {
        return Err(ArchiveError::decompression_failed(
            "Distilled",
            format!("invalid node count: {num_nodes}"),
        ));
    }
    if code_length == 0 || code_length > 16 {
        return Err(ArchiveError::decompression_failed(
            "Distilled",
            format!("invalid code length: {code_length}"),
        ));
    }

    // Read node data (packed, code_length bits each).
    let mut reader = BitReader::endian(&input[3..], LittleEndian);
    let mut node_data = Vec::with_capacity(num_nodes);

    for _ in 0..num_nodes {
        let node: u32 = reader.read_var(code_length as u32).map_err(|e| {
            ArchiveError::decompression_failed("Distilled", format!("read error: {e}"))
        })?;
        node_data.push(node as i32);
    }

    // Build symbol tree.
    let tree = HuffmanTree::build(&node_data, num_nodes)?;

    // Decompress data.
    let mut output = Vec::new();

    while let Ok(sym) = tree.decode(&mut reader) {
        if sym < EOF_SYMBOL {
            // Literal byte.
            output.push(sym as u8);
        } else if sym == EOF_SYMBOL {
            // Explicit EOF.
            break;
        } else {
            // Length code: length = symbol - 0x101 + 3.
            let length = (sym as usize).saturating_sub(0x101) + 3;

            // Decode offset base from static table.
            let offset_base = decode_offset_symbol(&mut reader)? as usize;

            // Read extra bits based on current position.
            let extra_bits = extra_bits_for_position(output.len());
            let extra: u32 = if extra_bits > 0 {
                reader.read_var(extra_bits).map_err(|e| {
                    ArchiveError::decompression_failed("Distilled", format!("read error: {e}"))
                })?
            } else {
                0
            };

            // Final offset = (base << extra_bits) + extra + 1.
            let offset = ((offset_base as u32) << extra_bits) + extra + 1;

            // Copy from back-reference.
            let start = output.len() as isize - offset as isize;
            for i in 0..length {
                let byte = if start + (i as isize) < 0 {
                    0x20 // Space for positions before start.
                } else {
                    output[(start + (i as isize)) as usize]
                };
                output.push(byte);
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(decompress(&[]).unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn test_extra_bits_thresholds() {
        assert_eq!(extra_bits_for_position(0), 0);
        assert_eq!(extra_bits_for_position(3), 0);
        assert_eq!(extra_bits_for_position(4), 1);
        assert_eq!(extra_bits_for_position(0x43), 1);
        assert_eq!(extra_bits_for_position(0x44), 2);
        assert_eq!(extra_bits_for_position(0xC3), 2);
        assert_eq!(extra_bits_for_position(0xC4), 3);
        assert_eq!(extra_bits_for_position(0x1C3), 3);
        assert_eq!(extra_bits_for_position(0x1C4), 4);
        assert_eq!(extra_bits_for_position(0x3C3), 4);
        assert_eq!(extra_bits_for_position(0x3C4), 5);
        assert_eq!(extra_bits_for_position(0x7C3), 5);
        assert_eq!(extra_bits_for_position(0x7C4), 6);
        assert_eq!(extra_bits_for_position(0xFC3), 6);
        assert_eq!(extra_bits_for_position(0xFC4), 7);
        assert_eq!(extra_bits_for_position(0xFFFF), 7);
    }
}
