//! Huffman decoding for ACE archives

use crate::error::{ArchiveError, Result};

use super::bitstream::BitStream;
use std::io::Read;

/// Maximum code width for Huffman trees
#[allow(dead_code)]
pub const MAX_CODE_WIDTH: u8 = 11;

/// Width-width bits for reading tree structure
const WIDTH_WIDTH_BITS: u8 = 3;
const MAX_WIDTH_WIDTH: u8 = 7;

/// Huffman tree for symbol decoding
pub struct HuffmanTree {
    /// Maps extended codes to symbols
    codes: Vec<u16>,
    /// Maps symbols to their bit widths
    widths: Vec<u8>,
    /// Maximum code width in this tree
    max_width: u8,
}

impl HuffmanTree {
    /// Read a symbol from the bit stream using this tree
    pub fn read_symbol<R: Read>(&self, bs: &mut BitStream<R>) -> Result<u16> {
        if self.max_width == 0 {
            return Ok(0);
        }

        let code = bs.peek_bits(self.max_width)?;
        if code as usize >= self.codes.len() {
            return Err(ArchiveError::decompression_failed(
                "huffman",
                "invalid code",
            ));
        }

        let symbol = self.codes[code as usize];
        let width = self.widths[symbol as usize];
        bs.skip_bits(width)?;

        Ok(symbol)
    }

    /// Build a Huffman tree from symbol widths
    pub fn from_widths(widths: &[u8], max_width: u8) -> Result<Self> {
        if max_width == 0 || widths.is_empty() {
            return Ok(Self {
                codes: vec![0],
                widths: vec![1],
                max_width: 1,
            });
        }

        let num_symbols = widths.len();
        let mut sorted_symbols: Vec<usize> = (0..num_symbols).collect();
        let mut sorted_widths: Vec<u8> = widths.to_vec();

        // Quicksort by width (descending) - must match original's unstable sort
        Self::quicksort(&mut sorted_widths, &mut sorted_symbols);

        // Count used symbols (non-zero widths)
        let mut used = 0;
        while used < sorted_widths.len() && sorted_widths[used] != 0 {
            used += 1;
        }

        // Handle degenerate cases - modify the original widths array
        let mut final_widths = widths.to_vec();
        if used < 2 {
            final_widths[sorted_symbols[0]] = 1;
            if used == 0 {
                used = 1;
            }
        }

        // Truncate to used symbols only (important!)
        sorted_symbols.truncate(used);
        sorted_widths.truncate(used);

        // Build codes table - iterate from lowest to highest width
        let mut codes = Vec::new();
        for i in (0..used).rev() {
            let sym = sorted_symbols[i];
            let wdt = sorted_widths[i];

            if wdt > max_width {
                return Err(ArchiveError::decompression_failed(
                    "huffman",
                    "width exceeds maximum",
                ));
            }

            let repeat = 1usize << (max_width - wdt);
            for _ in 0..repeat {
                codes.push(sym as u16);
            }

            let max_codes = 1usize << max_width;
            if codes.len() > max_codes {
                return Err(ArchiveError::decompression_failed(
                    "huffman",
                    "too many codes",
                ));
            }
        }

        Ok(Self {
            codes,
            widths: final_widths,
            max_width,
        })
    }

    /// Quicksort implementation matching the original ACE algorithm
    fn quicksort(keys: &mut [u8], values: &mut [usize]) {
        if keys.len() <= 1 {
            return;
        }
        Self::quicksort_range(keys, values, 0, keys.len() as isize - 1);
    }

    fn quicksort_range(keys: &mut [u8], values: &mut [usize], left: isize, right: isize) {
        let mut new_left = left;
        let mut new_right = right;
        let pivot = keys[right as usize];

        loop {
            while keys[new_left as usize] > pivot {
                new_left += 1;
            }
            while keys[new_right as usize] < pivot {
                new_right -= 1;
            }
            if new_left <= new_right {
                keys.swap(new_left as usize, new_right as usize);
                values.swap(new_left as usize, new_right as usize);
                new_left += 1;
                new_right -= 1;
            }
            if new_left >= new_right {
                break;
            }
        }

        if left < new_right {
            if left < new_right - 1 {
                Self::quicksort_range(keys, values, left, new_right);
            } else if keys[left as usize] < keys[new_right as usize] {
                keys.swap(left as usize, new_right as usize);
                values.swap(left as usize, new_right as usize);
            }
        }

        if right > new_left {
            if new_left < right - 1 {
                Self::quicksort_range(keys, values, new_left, right);
            } else if keys[new_left as usize] < keys[right as usize] {
                keys.swap(new_left as usize, right as usize);
                values.swap(new_left as usize, right as usize);
            }
        }
    }

    /// Read a Huffman tree from a bit stream
    pub fn read_from<R: Read>(
        bs: &mut BitStream<R>,
        max_width: u8,
        num_codes: usize,
    ) -> Result<Self> {
        let num_widths = (bs.read_bits(9)? + 1) as usize;
        let num_widths = num_widths.min(num_codes + 1);

        let lower_width = bs.read_bits(4)? as u8;
        let upper_width = bs.read_bits(4)? as u8;

        // Read width-of-widths
        let width_num_widths = (upper_width + 1) as usize;
        let mut width_widths = vec![0u8; width_num_widths];
        for i in 0..width_num_widths {
            width_widths[i] = bs.read_bits(WIDTH_WIDTH_BITS)? as u8;
        }

        let width_tree = Self::from_widths(&width_widths, MAX_WIDTH_WIDTH)?;

        // Read symbol widths
        let mut widths = Vec::with_capacity(num_widths);
        while widths.len() < num_widths {
            let symbol = width_tree.read_symbol(bs)?;

            if (symbol as u8) < upper_width {
                widths.push(symbol as u8);
            } else {
                let length = (bs.read_bits(4)? + 4) as usize;
                let length = length.min(num_widths - widths.len());
                widths.extend(std::iter::repeat(0u8).take(length));
            }
        }

        // Delta decode if upper_width > 0
        if upper_width > 0 {
            for i in 1..widths.len() {
                widths[i] = (widths[i] + widths[i - 1]) % upper_width;
            }
        }

        // Add lower_width offset
        for w in &mut widths {
            if *w > 0 {
                *w += lower_width;
            }
        }

        // Don't pad before from_widths - the sort must work with original length!
        let tree = Self::from_widths(&widths, max_width)?;
        Ok(tree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test based on Python docstring:
    /// >>> k, v = [1, 0, 0, 1, 2, 0, 0], list(range(7))
    /// >>> Huffman._quicksort(k, v)
    /// >>> (k, v)
    /// ([2, 1, 1, 0, 0, 0, 0], [4, 0, 3, 5, 6, 2, 1])
    #[test]
    fn test_quicksort() {
        let mut keys = [1u8, 0, 0, 1, 2, 0, 0];
        let mut values: Vec<usize> = (0..7).collect();

        HuffmanTree::quicksort(&mut keys, &mut values);

        println!("keys: {:?}", keys);
        println!("vals: {:?}", values);

        assert_eq!(keys, [2, 1, 1, 0, 0, 0, 0]);
        assert_eq!(values, [4, 0, 3, 5, 6, 2, 1]);
    }

    #[test]
    fn test_from_widths_real_data() {
        // From debug output: width_widths=[2, 3, 3, 4, 5, 4, 5, 4, 3, 3]
        let widths: Vec<u8> = vec![2, 3, 3, 4, 5, 4, 5, 4, 3, 3];
        let tree = HuffmanTree::from_widths(&widths, 7).unwrap();

        println!("codes len: {}", tree.codes.len());
        println!(
            "first 20 codes: {:?}",
            &tree.codes[..20.min(tree.codes.len())]
        );

        // Python result: codes len: 128, first 20 codes all 0
        assert_eq!(tree.codes.len(), 128);
        assert!(tree.codes[..20].iter().all(|&c| c == 0));
    }

    #[test]
    fn test_quicksort_with_real_widths() {
        // Test with the pattern from ace decompression
        let mut widths = vec![0u8; 284];
        // Set test widths: [(10, 8), (32, 6), (33, 11), (34, 7), (37, 11), (39, 11)]
        widths[10] = 8;
        widths[32] = 6;
        widths[33] = 11;
        widths[34] = 7;
        widths[37] = 11;
        widths[39] = 11;

        let mut sorted_symbols: Vec<usize> = (0..284).collect();
        let mut sorted_widths = widths.clone();
        HuffmanTree::quicksort(&mut sorted_widths, &mut sorted_symbols);

        // Count used
        let mut used = 0;
        while used < sorted_widths.len() && sorted_widths[used] != 0 {
            used += 1;
        }

        let first_used: Vec<_> = sorted_symbols[..used]
            .iter()
            .zip(sorted_widths[..used].iter())
            .collect();
        println!("First used (highest widths): {:?}", first_used);

        // Python: [(37, 11), (33, 11), (39, 11), (10, 8), (34, 7), (32, 6)]
        // Check that we get the same ordering
        assert_eq!(used, 6);
        // The symbols with width 11 should be 37, 33, 39 in that order
        assert_eq!(sorted_symbols[0], 37);
        assert_eq!(sorted_symbols[1], 33);
        assert_eq!(sorted_symbols[2], 39);
    }

    #[test]
    fn test_quicksort_ace_full_widths() {
        // Full widths from ACE file main huffman tree
        let mut widths = vec![0u8; 284];

        // From debug output
        let test_widths = [
            (10, 8),
            (32, 6),
            (33, 11),
            (34, 7),
            (37, 11),
            (39, 11),
            (40, 8),
            (41, 8),
            (44, 6),
            (45, 8),
            (46, 7),
            (47, 9),
            (48, 9),
            (49, 10),
            (50, 10),
            (51, 11),
            (52, 10),
            (53, 10),
            (54, 11),
            (55, 11),
            (56, 11),
            (57, 10),
            (58, 9),
            (59, 10),
            (65, 8),
            (66, 9),
            (67, 8),
            (68, 9),
            (69, 7),
            (70, 9),
            (71, 10),
            (72, 9),
            (73, 8),
            (74, 11),
            (75, 11),
            (76, 9),
            (77, 10),
            (78, 8),
            (79, 8),
            (80, 9),
            (82, 8),
            (83, 8),
            (84, 8),
            (85, 8),
            (86, 11),
            (87, 9),
            (88, 11),
            (89, 10),
            (91, 10),
            (93, 10),
            (95, 7),
            (97, 7),
            (98, 8),
            (99, 7),
            (100, 7),
            (101, 6),
            (102, 8),
            (103, 9),
            (104, 8),
            (105, 7),
            (106, 11),
            (108, 7),
            (109, 8),
            (110, 6),
            (111, 6),
            (112, 8),
            (113, 11),
            (114, 6),
            (115, 7),
            (116, 6),
            (117, 8),
            (118, 9),
            (119, 8),
            (120, 11),
            (121, 9),
            (122, 11),
            (256, 6),
            (257, 4),
            (258, 4),
            (259, 5),
            (260, 5),
            (261, 5),
            (262, 5),
            (263, 6),
            (264, 6),
            (265, 6),
            (266, 7),
            (267, 6),
            (268, 7),
            (269, 8),
            (270, 8),
            (271, 10),
            (272, 11),
            (273, 11),
            (274, 11),
            (275, 10),
            (276, 10),
            (277, 11),
            (278, 11),
            (279, 10),
        ];

        for (i, w) in test_widths.iter() {
            widths[*i] = *w;
        }

        let mut sorted_symbols: Vec<usize> = (0..widths.len()).collect();
        let mut sorted_widths = widths.clone();
        HuffmanTree::quicksort(&mut sorted_widths, &mut sorted_symbols);

        // Count used
        let mut used = 0;
        while used < sorted_widths.len() && sorted_widths[used] != 0 {
            used += 1;
        }

        println!("Rust sorted (first 20 used):");
        for i in 0..20.min(used) {
            println!(
                "  [{i}] symbol={}, width={}",
                sorted_symbols[i], sorted_widths[i]
            );
        }

        // Python result (first 20):
        // [0] symbol=39, width=11
        // [1] symbol=120, width=11
        // [2] symbol=51, width=11
        // [3] symbol=106, width=11
        // [4] symbol=33, width=11
        // [5] symbol=37, width=11
        // [6] symbol=54, width=11
        // [7] symbol=74, width=11
        assert_eq!(sorted_symbols[0], 39);
        assert_eq!(sorted_symbols[1], 120);
        assert_eq!(sorted_symbols[2], 51);
        assert_eq!(sorted_symbols[3], 106);
    }
}
