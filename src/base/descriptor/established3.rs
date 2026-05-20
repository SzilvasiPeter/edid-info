//! Established Timings III Descriptor.
//!
//! A bitmap of supported additional standard timings.
//! Uses tag 0xF7 and version 10.

use crate::base::dmt::{Dmt, find_dmt};
use crate::common::DESC_LEN;

const VERSION: u8 = 10;
const MAP: [(usize, u8, u8); 44] = [
    (0, 0x80, 0x01),
    (0, 0x40, 0x02),
    (0, 0x20, 0x03),
    (0, 0x10, 0x07),
    (0, 0x08, 0x0E),
    (0, 0x04, 0x0C),
    (0, 0x02, 0x13),
    (0, 0x01, 0x15),
    (1, 0x80, 0x16),
    (1, 0x40, 0x17),
    (1, 0x20, 0x18),
    (1, 0x10, 0x19),
    (1, 0x08, 0x20),
    (1, 0x04, 0x21),
    (1, 0x02, 0x23),
    (1, 0x01, 0x25),
    (2, 0x80, 0x27),
    (2, 0x40, 0x2E),
    (2, 0x20, 0x2F),
    (2, 0x10, 0x30),
    (2, 0x08, 0x31),
    (2, 0x04, 0x29),
    (2, 0x02, 0x2A),
    (2, 0x01, 0x2B),
    (3, 0x80, 0x2C),
    (3, 0x40, 0x39),
    (3, 0x20, 0x3A),
    (3, 0x10, 0x3B),
    (3, 0x08, 0x3C),
    (3, 0x04, 0x33),
    (3, 0x02, 0x34),
    (3, 0x01, 0x35),
    (4, 0x80, 0x36),
    (4, 0x40, 0x37),
    (4, 0x20, 0x3E),
    (4, 0x10, 0x3F),
    (4, 0x08, 0x41),
    (4, 0x04, 0x42),
    (4, 0x02, 0x44),
    (4, 0x01, 0x45),
    (5, 0x80, 0x46),
    (5, 0x40, 0x47),
    (5, 0x20, 0x49),
    (5, 0x10, 0x4A),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Established3 {
    timings: [Option<Dmt>; 44],
}

impl Established3 {
    #[must_use]
    pub(super) const fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
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

        let mut timings = [None; 44];
        let mut i = 0;
        while i < MAP.len() {
            let (byte, mask, dmt_id) = MAP[i];
            if (raw[6 + byte] & mask) != 0 {
                timings[i] = find_dmt(dmt_id);
            }
            i += 1;
        }
        Some(Self { timings })
    }

    #[must_use]
    pub const fn timings(&self) -> [Option<Dmt>; 44] {
        self.timings
    }
}
