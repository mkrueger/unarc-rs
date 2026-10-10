pub const PAYLOAD: &[u8] = b"hello Amiga";

pub fn put(block: &mut [u8], index: usize, value: u32) {
    block[index * 4..index * 4 + 4].copy_from_slice(&value.to_be_bytes());
}

pub fn get(block: &[u8], index: usize) -> u32 {
    u32::from_be_bytes(block[index * 4..index * 4 + 4].try_into().unwrap())
}

pub fn fix_checksum(block: &mut [u8], index: usize) {
    put(block, index, 0);
    let sum = block
        .as_chunks::<4>()
        .0
        .iter()
        .fold(0u32, |sum, bytes| sum.wrapping_add(u32::from_be_bytes(*bytes)));
    put(block, index, sum.wrapping_neg());
}

pub fn named_header(size: usize, block: u32, parent: u32, subtype: i32, name: &[u8]) -> Vec<u8> {
    let mut data = vec![0; size];
    put(&mut data, 0, 2);
    put(&mut data, 1, block);
    put(&mut data, size / 4 - 3, parent);
    put(&mut data, size / 4 - 1, subtype as u32);
    data[size - 80] = name.len() as u8;
    data[size - 79..size - 79 + name.len()].copy_from_slice(name);
    data
}

/// An independently constructed filesystem with a directory and a small file.
pub fn image(dos_type: u8, size: usize, blocks: usize) -> Vec<u8> {
    let mut image = vec![0; size * blocks];
    image[..3].copy_from_slice(b"DOS");
    image[3] = dos_type;
    let root = blocks / 2;
    let mut root_data = named_header(size, 0, 0, 1, b"Test");
    put(&mut root_data, 3, (size / 4 - 56) as u32);
    put(&mut root_data, 6, 3);
    fix_checksum(&mut root_data, 5);
    image[root * size..(root + 1) * size].copy_from_slice(&root_data);
    let mut directory = named_header(size, 3, root as u32, 2, b"docs");
    put(&mut directory, 6, 4);
    fix_checksum(&mut directory, 5);
    image[3 * size..4 * size].copy_from_slice(&directory);
    let mut file = named_header(size, 4, 3, -3, b"readme");
    put(&mut file, 2, 1);
    put(&mut file, 4, 5);
    put(&mut file, size / 4 - 47, PAYLOAD.len() as u32);
    put(&mut file, size / 4 - 51, 5);
    fix_checksum(&mut file, 5);
    image[4 * size..5 * size].copy_from_slice(&file);
    let data = &mut image[5 * size..6 * size];
    if dos_type & 1 == 0 {
        put(data, 0, 8);
        put(data, 1, 4);
        put(data, 2, 1);
        put(data, 3, PAYLOAD.len() as u32);
        data[24..24 + PAYLOAD.len()].copy_from_slice(PAYLOAD);
        fix_checksum(data, 5);
    } else {
        data[..PAYLOAD.len()].copy_from_slice(PAYLOAD);
    }
    image
}
