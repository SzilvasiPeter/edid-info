//! Standard Timing Identification Level 3.
//!
//! A bitmap of supported additional standard timings.
//! Uses tag 0xF7 and version 0x10.
//! Contains 48 bits (6 bytes) representing predefined timing codes.

use crate::common::DESC_LEN;

const VERSION: u8 = 0x10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Std3 {
    map: [u8; 6],
}

impl Std3 {
    #[must_use]
    pub(super) fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        // TODO: Move this to the `monitor::validate` function
        if raw[5] != VERSION {
            return None;
        }
        // TODO: Move this to the `monitor::validate` function
        if raw[12] != 0
            || raw[13] != 0
            || raw[14] != 0
            || raw[15] != 0
            || raw[16] != 0
            || raw[17] != 0
        {
            return None;
        }

        // TODO: We should considilate the stardand timings into a single map, to avoid duplication
        let mut map = [0; 6];
        map.copy_from_slice(&raw[6..12]);
        Some(Self { map })
    }

    #[must_use]
    pub const fn map(&self) -> [u8; 6] {
        self.map
    }

    // TODO: move this to test module
    #[must_use]
    pub const fn byte(&self, i: usize) -> Option<u8> {
        if i < 6 { Some(self.map[i]) } else { None }
    }

    // TODO: move this to test module
    #[must_use]
    pub const fn has(&self, byte: usize, bit: u8) -> Option<bool> {
        if byte >= 6 || bit > 7 {
            return None;
        }
        Some((self.map[byte] & (1 << bit)) != 0)
    }
}
