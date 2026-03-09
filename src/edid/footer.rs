//! EDID block footer (bytes 126–127).
//!
//! Contains the extension count and checksum for the block.
//!
//! # Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 126  | Number of extension blocks following this block |
//! | 127  | Checksum (sum of all 128 bytes must equal 0) |

use crate::edid::BLOCK_LEN;

pub const FOOTER_OFF: usize = 126;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Footer {
    extension_num: u8,
    checksum: u8,
}

impl Footer {
    #[must_use]
    pub const fn parse(raw: &[u8; BLOCK_LEN]) -> Self {
        Self {
            extension_num: raw[FOOTER_OFF],
            checksum: raw[FOOTER_OFF + 1],
        }
    }

    #[must_use]
    pub const fn extension_num(&self) -> u8 {
        self.extension_num
    }

    #[must_use]
    pub const fn checksum(&self) -> u8 {
        self.checksum
    }

    #[must_use]
    pub fn checksum_ok(raw: &[u8; BLOCK_LEN]) -> bool {
        crate::edid::check(raw)
    }
}
