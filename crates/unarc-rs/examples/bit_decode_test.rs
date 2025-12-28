fn main() {
    let data = include_bytes!("../tests/uc2/normal.uc2");
    let compressed = &data[29..]; // Skip to compressed data

    println!(
        "First 4 bytes: {:02x} {:02x} {:02x} {:02x}",
        compressed[0], compressed[1], compressed[2], compressed[3]
    );

    // Little-endian u16
    let word0 = u16::from_le_bytes([compressed[0], compressed[1]]);
    let word1 = u16::from_le_bytes([compressed[2], compressed[3]]);

    println!("word0 = 0x{:04x} = {:016b}", word0, word0);
    println!("word1 = 0x{:04x} = {:016b}", word1, word1);

    // After BitReader::new(), we have:
    // word_buf = word0
    // bits_left = 16
    // Then skip(13):
    //   - Shifts preview left 13 bits
    //   - Adds top 13 bits from word_buf
    //   - word_buf shifts, may need refill

    // Simulate this:
    let mut word_buf = word0;
    let mut bits_left = 16u8;
    let mut preview = 0u16;

    // skip(13)
    {
        let bits_from_word = 13.min(bits_left);
        let mask = (1u16 << bits_from_word) - 1;
        let shift_amount = 16 - bits_from_word;
        let new_bits = (word_buf >> shift_amount) & mask;
        preview = (preview << bits_from_word) | new_bits;
        word_buf <<= bits_from_word;
        bits_left -= bits_from_word;
        preview &= (1 << 13) - 1; // Keep only 13 bits

        println!("After skip(13): preview=0x{:04x} ({:013b}), bits_left={}", preview, preview, bits_left);
    }

    // Now get(1) for has_block
    {
        let n = 1;
        let result = preview >> (13 - n);
        println!("get(1) for has_block: bit = {}", result);

        // skip(1)
        preview = (preview << 1) & ((1 << 13) - 1);
        // Need to add 1 bit from word_buf
        if bits_left == 0 {
            word_buf = word1;
            bits_left = 16;
        }
        let new_bit = (word_buf >> 15) & 1;
        preview |= new_bit;
        bits_left -= 1;

        println!("After get(1): preview=0x{:04x} ({:013b}), bits_left={}", preview, preview, bits_left);
    }

    // Now get(1) for has_tree
    {
        let n = 1;
        let result = preview >> (13 - n);
        println!("get(1) for has_tree: bit = {}", result);
    }
}
