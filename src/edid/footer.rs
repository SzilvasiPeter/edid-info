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

pub const FOOTER_OFF: usize = 126;
pub const FOOTER_LEN: usize = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Footer {
    extension_num: u8,
    checksum: u8,
}

impl Footer {
    #[must_use]
    pub const fn parse(raw: &[u8; FOOTER_LEN]) -> Self {
        Self {
            extension_num: raw[0],
            checksum: raw[1],
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
}
