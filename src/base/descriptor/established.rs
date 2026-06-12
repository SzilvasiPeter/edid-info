//! Established Timings III Descriptor.
//!
//! A bitmap of supported additional standard timings.
//! Uses tag 0xF7 and version 10.

use crate::base::dmt::Dmt;
use crate::base::established::flag_dmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstablishedTimings {
    bytes: [u8; 6],
}

impl EstablishedTimings {
    pub(super) const fn parse(raw: &[u8; 13]) -> Self {
        Self {
            bytes: [raw[1], raw[2], raw[3], raw[4], raw[5], raw[6]],
        }
    }

    /// Supported established timings.
    pub fn iter(&self) -> impl Iterator<Item = Dmt> {
        let bytes = self.bytes;
        [
            flag_dmt(bytes[0], 0x80, 0x01),
            flag_dmt(bytes[0], 0x40, 0x02),
            flag_dmt(bytes[0], 0x20, 0x03),
            flag_dmt(bytes[0], 0x10, 0x07),
            flag_dmt(bytes[0], 0x08, 0x0E),
            flag_dmt(bytes[0], 0x04, 0x0C),
            flag_dmt(bytes[0], 0x02, 0x13),
            flag_dmt(bytes[0], 0x01, 0x15),
            flag_dmt(bytes[1], 0x80, 0x16),
            flag_dmt(bytes[1], 0x40, 0x17),
            flag_dmt(bytes[1], 0x20, 0x18),
            flag_dmt(bytes[1], 0x10, 0x19),
            flag_dmt(bytes[1], 0x08, 0x20),
            flag_dmt(bytes[1], 0x04, 0x21),
            flag_dmt(bytes[1], 0x02, 0x23),
            flag_dmt(bytes[1], 0x01, 0x25),
            flag_dmt(bytes[2], 0x80, 0x27),
            flag_dmt(bytes[2], 0x40, 0x2E),
            flag_dmt(bytes[2], 0x20, 0x2F),
            flag_dmt(bytes[2], 0x10, 0x30),
            flag_dmt(bytes[2], 0x08, 0x31),
            flag_dmt(bytes[2], 0x04, 0x29),
            flag_dmt(bytes[2], 0x02, 0x2A),
            flag_dmt(bytes[2], 0x01, 0x2B),
            flag_dmt(bytes[3], 0x80, 0x2C),
            flag_dmt(bytes[3], 0x40, 0x39),
            flag_dmt(bytes[3], 0x20, 0x3A),
            flag_dmt(bytes[3], 0x10, 0x3B),
            flag_dmt(bytes[3], 0x08, 0x3C),
            flag_dmt(bytes[3], 0x04, 0x33),
            flag_dmt(bytes[3], 0x02, 0x34),
            flag_dmt(bytes[3], 0x01, 0x35),
            flag_dmt(bytes[4], 0x80, 0x36),
            flag_dmt(bytes[4], 0x40, 0x37),
            flag_dmt(bytes[4], 0x20, 0x3E),
            flag_dmt(bytes[4], 0x10, 0x3F),
            flag_dmt(bytes[4], 0x08, 0x41),
            flag_dmt(bytes[4], 0x04, 0x42),
            flag_dmt(bytes[4], 0x02, 0x44),
            flag_dmt(bytes[4], 0x01, 0x45),
            flag_dmt(bytes[5], 0x80, 0x46),
            flag_dmt(bytes[5], 0x40, 0x47),
            flag_dmt(bytes[5], 0x20, 0x49),
            flag_dmt(bytes[5], 0x10, 0x4A),
        ]
        .into_iter()
        .flatten()
    }
}
