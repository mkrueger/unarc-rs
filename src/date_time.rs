#[derive(Clone, Copy, PartialEq)]
pub struct DosDateTime(u32);

impl DosDateTime {
    pub fn new(date_time_modified: u32) -> Self {
        Self(date_time_modified)
    }

    pub fn year(&self) -> u16 {
        ((self.0 >> 25) & 0x7F) as u16 + 1980
    }

    pub fn month(&self) -> u8 {
        ((self.0 >> 21) & 0xF) as u8
    }

    pub fn day(&self) -> u8 {
        ((self.0 >> 16) & 0x1F) as u8
    }

    pub fn hour(&self) -> u8 {
        ((self.0 >> 11) & 0x1F) as u8
    }

    pub fn minute(&self) -> u8 {
        ((self.0 >> 5) & 0x3F) as u8
    }

    pub fn second(&self) -> u8 {
        (self.0 & 0x1F) as u8 * 2
    }
}
