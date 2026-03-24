//! Extension flag and checksum (bytes 126–127).
//!
//! Contains the extension count and checksum for the block.
//!
//! # Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 126  | Number of Extension blocks |
//! | 127  | Checksum |

/// Footer offset in the base block.
pub const FOOTER_OFF: usize = 126;

/// Footer length in bytes.
pub const FOOTER_LEN: usize = 2;

/// Footer structure containing extension count and checksum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Footer {
    extension_num: u8,
    checksum: u8,
}

impl Footer {
    /// Parses a footer from 2 raw bytes.
    #[must_use]
    pub const fn new(raw: &[u8; FOOTER_LEN]) -> Self {
        Self {
            extension_num: raw[0],
            checksum: raw[1],
        }
    }

    /// Number of extensions to follow. 0 if no extensions.
    #[must_use]
    pub const fn extension_num(&self) -> u8 {
        self.extension_num
    }

    /// Checksum. Sum of all 128 bytes should equal 0 (mod 256).
    #[must_use]
    pub const fn checksum(&self) -> u8 {
        self.checksum
    }
}
