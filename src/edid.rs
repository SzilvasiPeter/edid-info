//! EDID parsing for the 128-byte [`Base`] block and [`Extension`]s.
//!
//! Parse flow: validate length, parse base block, then parse
//! extensions and keep unknown blocks as raw bytes.
//!
//! # References
//! - [Extended Display Identification Wikipedia](https://en.wikipedia.org/wiki/Extended_Display_Identification_Data)
//! - [VESA Enhanced Extended Display Identification Data Standard](https://glenwing.github.io/docs/VESA-EEDID-A2.pdf)

use crate::base::Base;
use crate::common::{BLOCK_LEN, FailureKind, Validation};
use crate::extensions::Extension;

/// Maximum number of extension block.
pub const MAX_EXT: usize = 64;

/// Unrecoverable errors that prevent parsing the EDID data at all.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    InvalidLen,
    BadHeader,
    ExtCountTooLarge,
}

/// Parsed EDID data with the base block and optional extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid<'a> {
    raw: &'a [u8],
    ext_len: usize,
}

impl<'a> Edid<'a> {
    /// Parses raw EDID bytes.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError::InvalidLen`] for input shorter than one block.
    /// Returns [`ParseError::BadHeader`] for an invalid header pattern.
    /// Returns [`ParseError::ExtCountTooLarge`] for more than the max blocks.
    pub fn parse(raw: &'a [u8]) -> Result<Self, ParseError> {
        if raw.len() < BLOCK_LEN || !raw.len().is_multiple_of(BLOCK_LEN) {
            return Err(ParseError::InvalidLen);
        }

        if raw[0..8] != [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00] {
            return Err(ParseError::BadHeader);
        }

        if raw.len() > BLOCK_LEN * (MAX_EXT + 1) {
            return Err(ParseError::ExtCountTooLarge);
        }

        let ext_len = (raw.len() / BLOCK_LEN) - 1;
        Ok(Self { raw, ext_len })
    }

    /// Returns the base block.
    ///
    /// # Panics
    ///
    /// Panics if the internal buffer is shorter than 128 bytes.
    /// This should never happen because `Edid::parse` validates the length.
    #[must_use]
    pub fn base(&self) -> Base<'_> {
        let base_raw: &[u8; BLOCK_LEN] = self.raw[..BLOCK_LEN].try_into().unwrap();
        Base::new(base_raw)
    }

    /// Returns an iterator over all available extension blocks.
    ///
    /// # Panics
    ///
    /// Panics if the internal buffer is shorter than expected.
    /// This should never happen because `Edid::parse` validates the length.
    pub fn extensions(&self) -> impl Iterator<Item = Extension<'_>> + '_ {
        let ext_len = self.ext_len;
        (0..ext_len).map(move |i| {
            let off = BLOCK_LEN * (i + 1);
            let block: &[u8; BLOCK_LEN] = self.raw[off..off + BLOCK_LEN].try_into().unwrap();
            Extension::parse(block)
        })
    }

    /// Validates the all (base and extension(s) blocks) EDID data.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let base = self.base();
        let ext_num = base.footer().extension_num() as usize;
        Validation::new()
            .then(base.validate())
            .fail_if(ext_num != self.ext_len, FailureKind::BaseExtCountMismatch)
    }
}
