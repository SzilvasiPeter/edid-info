//! Standard Timing Identification Level 3.
//!
//! A bitmap of supported additional standard timings.
//! Uses tag 0xF7 and version 0x10.
//! Contains 48 bits (6 bytes) representing predefined timing codes.

use crate::edid::descriptor::DESC_LEN;

const VERSION: u8 = 0x10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Std3 {
    map: [u8; 6],
}

impl Std3 {
    #[must_use]
    pub fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        if raw[0] != 0 || raw[1] != 0 || raw[2] != 0 || raw[3] != 0xF7 || raw[4] != 0 {
            return None;
        }
        if raw[5] != VERSION {
            return None;
        }
        if raw[12] != 0
            || raw[13] != 0
            || raw[14] != 0
            || raw[15] != 0
            || raw[16] != 0
            || raw[17] != 0
        {
            return None;
        }
        let mut map = [0; 6];
        map.copy_from_slice(&raw[6..12]);
        Some(Self { map })
    }

    #[must_use]
    pub const fn map(&self) -> [u8; 6] {
        self.map
    }

    #[must_use]
    pub const fn byte(&self, i: usize) -> Option<u8> {
        if i < 6 { Some(self.map[i]) } else { None }
    }

    #[must_use]
    pub const fn has(&self, byte: usize, bit: u8) -> Option<bool> {
        if byte >= 6 || bit > 7 {
            return None;
        }
        Some((self.map[byte] & (1 << bit)) != 0)
    }
}
