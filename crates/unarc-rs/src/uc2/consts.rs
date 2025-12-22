//! Constants for UC2 decompression
//! Based on ULTRACMP.CPP from the original UC2 source code

pub const MAX_CODE_BITS: usize = 13;
pub const LOOKUP_SIZE: usize = 1 << MAX_CODE_BITS; // 8192

// Literal/Distance symbols
pub const NUM_BYTE_SYM: usize = 256;
pub const NUM_DIST_SYM: usize = 60;
pub const NUM_LD_SYM: usize = NUM_BYTE_SYM + NUM_DIST_SYM; // 316

// Length symbols
pub const NUM_LEN_SYM: usize = 28;

// Tree encoding symbols
pub const NUM_LEN_CODES: usize = 15;

/// Pack distance code: base_dist | (extra_bits << 20) | (1 << 16)
const fn d(base: u32, extra_bits: u32) -> u32 {
    base | (extra_bits << 20) | (1 << 16)
}

/// Packed distance codes for Huffman table
pub const PACKED_DIST_CODES: [u32; NUM_DIST_SYM] = [
    // 0-14: base 1-15, 0 extra bits
    d(1, 0),
    d(2, 0),
    d(3, 0),
    d(4, 0),
    d(5, 0),
    d(6, 0),
    d(7, 0),
    d(8, 0),
    d(9, 0),
    d(10, 0),
    d(11, 0),
    d(12, 0),
    d(13, 0),
    d(14, 0),
    d(15, 0),
    // 15-29: base 16,32,..,240, 4 extra bits
    d(16, 4),
    d(32, 4),
    d(48, 4),
    d(64, 4),
    d(80, 4),
    d(96, 4),
    d(112, 4),
    d(128, 4),
    d(144, 4),
    d(160, 4),
    d(176, 4),
    d(192, 4),
    d(208, 4),
    d(224, 4),
    d(240, 4),
    // 30-44: base 256,512,..,3840, 8 extra bits
    d(256, 8),
    d(512, 8),
    d(768, 8),
    d(1024, 8),
    d(1280, 8),
    d(1536, 8),
    d(1792, 8),
    d(2048, 8),
    d(2304, 8),
    d(2560, 8),
    d(2816, 8),
    d(3072, 8),
    d(3328, 8),
    d(3584, 8),
    d(3840, 8),
    // 45-59: base 4096,8192,..,61440, 12 extra bits
    d(4096, 12),
    d(8192, 12),
    d(12288, 12),
    d(16384, 12),
    d(20480, 12),
    d(24576, 12),
    d(28672, 12),
    d(32768, 12),
    d(36864, 12),
    d(40960, 12),
    d(45056, 12),
    d(49152, 12),
    d(53248, 12),
    d(57344, 12),
    d(61440, 12),
];

/// Length code table - maps symbol to (base_length, extra_bits)
pub const LEN_CODES: [(u32, u8); NUM_LEN_SYM] = [
    (3, 0),
    (4, 0),
    (5, 0),
    (6, 0),
    (7, 0),
    (8, 0),
    (9, 0),
    (10, 0),
    (11, 1),
    (13, 1),
    (15, 1),
    (17, 1),
    (19, 1),
    (21, 1),
    (23, 1),
    (25, 1),
    (27, 3),
    (35, 3),
    (43, 3),
    (51, 3),
    (59, 3),
    (67, 3),
    (75, 3),
    (83, 3),
    (91, 6),
    (155, 9),
    (667, 11),
    (2715, 15),
];

/// Delta decoding table for tree lengths
pub const VVAL: [[u8; 14]; 14] = [
    [0, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0],
    [2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0],
    [3, 2, 4, 1, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0],
    [4, 3, 5, 2, 6, 1, 7, 8, 9, 10, 11, 12, 13, 0],
    [5, 4, 6, 3, 7, 2, 8, 1, 9, 10, 11, 12, 13, 0],
    [6, 5, 7, 4, 8, 3, 9, 2, 10, 1, 11, 12, 13, 0],
    [7, 6, 8, 5, 9, 4, 10, 3, 11, 2, 12, 1, 13, 0],
    [8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 0, 1],
    [9, 8, 10, 7, 11, 6, 12, 5, 13, 4, 0, 3, 2, 1],
    [10, 9, 11, 8, 12, 7, 13, 6, 0, 5, 4, 3, 2, 1],
    [11, 10, 12, 9, 13, 8, 0, 7, 6, 5, 4, 3, 2, 1],
    [12, 11, 13, 10, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1],
    [13, 12, 0, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
];

/// RLE patterns for tree length decoding
pub const RLE_PATTERNS: [[u16; 8]; 4] = [
    [0x009, 0x202, 0x1, 0x202, 0x12, 0x260, 0x80, 0x258],
    [0x280, 0x80, 0x258, 0, 0, 0, 0, 0],
    [0x009, 0x202, 0x1, 0x202, 0x12, 0x338, 0, 0],
    [0x358, 0, 0, 0, 0, 0, 0, 0],
];

/// Default lengths RLE-encoded data
pub const DEFAULT_LENGTHS_RLE: &[(usize, u8)] = &[
    (10, 9),
    (1, 7),
    (1, 9),
    (1, 7),
    (19, 9),
    (1, 7),
    (13, 8),
    (1, 7),
    (11, 8),
    (1, 7),
    (33, 8),
    (1, 7),
    (35, 8),
    (128, 10),
    (16, 6),
    (12, 7),
    (6, 8),
    (10, 9),
    (16, 10),
    (9, 4),
    (9, 5),
    (10, 6),
];

// Tree decoding constants
pub const NUM_LO_ASCII: usize = 28;
pub const NUM_HI_BYTE: usize = 128;
pub const REPEAT_CODE: usize = 14;
pub const MIN_REPEAT: usize = 6;

/// End of block marker distance value
pub const EOB_MARK: u32 = 125 * 512 + 1; // 64001
