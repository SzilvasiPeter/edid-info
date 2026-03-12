//! CTA Extension Block header.
//!
//! The 4-byte header at the start of every CTA-861 extension block.
//!
//! # Header Structure (4 bytes)
//!
//! | Byte | Description |
//! |------|-------------|
//! | 0    | Tag (always 0x02) |
//! | 1    | Revision number |
//! | 2    | Offset to DTDs (0 if none) |
//! | 3    | Flags: underscan, basic audio, YCbCr 4:4:4, YCbCr 4:2:2, native DTD count |

use crate::edid::BLOCK_LEN;
use crate::edid::bits::is_set;

pub const CTA_TAG: u8 = 0x02;

/// CTA Extension Block header structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    tag: u8,
    rev: u8,
    dtd_off: u8,
    flags: u8,
}

impl Header {
    #[must_use]
    pub fn parse(raw: &[u8; BLOCK_LEN]) -> Option<Self> {
        (raw[0] == CTA_TAG).then_some(Self {
            tag: raw[0],
            rev: raw[1],
            dtd_off: raw[2],
            flags: raw[3],
        })
    }

    #[must_use]
    pub const fn tag(&self) -> u8 {
        self.tag
    }

    #[must_use]
    pub const fn rev(&self) -> u8 {
        self.rev
    }

    #[must_use]
    pub const fn dtd_off(&self) -> u8 {
        self.dtd_off
    }

    #[must_use]
    pub const fn underscan(&self) -> bool {
        is_set(self.flags, 7)
    }

    #[must_use]
    pub const fn basic_audio(&self) -> bool {
        is_set(self.flags, 6)
    }

    #[must_use]
    pub const fn ycbcr_444(&self) -> bool {
        is_set(self.flags, 5)
    }

    #[must_use]
    pub const fn ycbcr_422(&self) -> bool {
        is_set(self.flags, 4)
    }

    #[must_use]
    pub const fn native_dtd_num(&self) -> u8 {
        self.flags & 0b0000_1111
    }
}
