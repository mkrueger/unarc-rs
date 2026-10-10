struct AceBits(Vec<bool>);

impl AceBits {
    fn push(&mut self, value: u32, width: u32) {
        for bit in (0..width).rev() {
            self.0.push(value & (1 << bit) != 0);
        }
    }

    fn single_symbol_tree(&mut self, symbol: u32) {
        self.push(symbol, 9);
        self.push(0, 4);
        self.push(2, 4);
        for width in [1, 1, 0] {
            self.push(width, 3);
        }
        // Delta widths are zero until the selected symbol; this width tree encodes 0 as 1.
        for index in 0..=symbol {
            self.push(u32::from(index != symbol), 1);
        }
    }

    fn finish(self) -> Vec<u8> {
        let mut bytes = vec![0u8; self.0.len().div_ceil(32) * 4];
        for (index, bit) in self.0.into_iter().enumerate() {
            if bit {
                let word = index / 32 * 4;
                let value = u32::from_le_bytes(bytes[word..word + 4].try_into().unwrap());
                bytes[word..word + 4].copy_from_slice(&(value | (1 << (31 - index % 32))).to_le_bytes());
            }
        }
        bytes
    }
}

fn ace_lz77_payload(symbol: u32, symbols_in_block: u32) -> Vec<u8> {
    let mut bits = AceBits(Vec::new());
    bits.single_symbol_tree(symbol);
    bits.single_symbol_tree(0);
    bits.push(symbols_in_block, 15);
    bits.push(0, 1);
    if symbol == 260 {
        // Explicit distance zero means a distance-one copy; length symbol zero means two bytes.
        bits.push(0, 1);
    }
    bits.finish()
}

fn ace_header(data: &[u8]) -> Vec<u8> {
    let mut bytes = ((!crc32fast::hash(data)) as u16).to_le_bytes().to_vec();
    bytes.extend_from_slice(&(data.len() as u16).to_le_bytes());
    bytes.extend_from_slice(data);
    bytes
}

pub fn ace_main_header(flags: u16) -> Vec<u8> {
    let mut data = vec![0];
    data.extend_from_slice(&flags.to_le_bytes());
    data.extend_from_slice(b"**ACE**");
    data.extend_from_slice(&[10, 10, 0, 0]);
    data.extend_from_slice(&[0; 12]);
    ace_header(&data)
}

fn ace_member(name: &str, method: u8, packed: &[u8], output: &[u8], flags: u16) -> Vec<u8> {
    let mut data = vec![1];
    data.extend_from_slice(&flags.to_le_bytes());
    data.extend_from_slice(&(packed.len() as u32).to_le_bytes());
    data.extend_from_slice(&(output.len() as u32).to_le_bytes());
    data.extend_from_slice(&[0; 8]);
    data.extend_from_slice(&(!crc32fast::hash(output)).to_le_bytes());
    data.extend_from_slice(&[method, 0]);
    data.extend_from_slice(&[0; 4]);
    data.extend_from_slice(&(name.len() as u16).to_le_bytes());
    data.extend_from_slice(name.as_bytes());
    let mut bytes = ace_header(&data);
    bytes.extend_from_slice(packed);
    bytes
}

pub fn synthetic_ace(solid: bool, stored_prefix: bool) -> Vec<u8> {
    let mut bytes = ace_main_header(if solid { 0x8000 } else { 0 });
    let first = if stored_prefix { b"A".to_vec() } else { ace_lz77_payload(65, 2) };
    bytes.extend(ace_member("first.txt", u8::from(!stored_prefix), &first, b"A", 1));
    let second = ace_lz77_payload(if solid { 260 } else { 66 }, 1);
    bytes.extend(ace_member("second.txt", 1, &second, if solid { b"AA" } else { b"B" }, 1));
    bytes
}

pub fn synthetic_encrypted_ace(solid: bool) -> Vec<u8> {
    let mut bytes = ace_main_header(if solid { 0x8000 } else { 0 });
    // "AAAAAAAA", Blowfish-CBC with a zero IV and the ACE SHA-1 key for password "test".
    let encrypted = [0x48, 0x75, 0x48, 0xa7, 0x7c, 0xd6, 0x73, 0x5d];
    bytes.extend(ace_member("first.txt", 0, &encrypted, b"AAAAAAAA", 0x81));
    let second = ace_lz77_payload(if solid { 260 } else { 66 }, 1);
    bytes.extend(ace_member("second.txt", 1, &second, if solid { b"AA" } else { b"B" }, 1));
    bytes
}
