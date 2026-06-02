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

use crate::common::BLOCK_LEN;

/// Footer offset in the base block.
pub const FOOTER_OFF: usize = 126;

/// Footer length in bytes.
pub const FOOTER_LEN: usize = 2;

/// Footer structure containing extension count and checksum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Footer {
    extension_count: u8,
    checksum: u8,
}

impl Footer {
    /// Initialize a footer from base block bytes.
    ///
    /// Byte sizes:
    /// - `extension_count`: 1 byte
    /// - `checksum`: 1 byte
    #[must_use]
    pub const fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        Self {
            extension_count: raw[FOOTER_OFF],
            checksum: raw[FOOTER_OFF + 1],
        }
    }

    /// Number of extensions to follow. 0 if no extensions.
    #[must_use]
    pub const fn extension_count(&self) -> u8 {
        self.extension_count
    }

    /// Checksum. Sum of all 128 bytes should equal 0 (mod 256).
    #[must_use]
    pub const fn checksum(&self) -> u8 {
        self.checksum
    }
}

impl core::fmt::Display for Footer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Extensions: {}, Checksum: 0x{:02X}",
            self.extension_count, self.checksum
        )
    }
}
