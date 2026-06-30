//! EDID parsing for the 128-byte [`Base`] block and [`Extension`]s.
//!
//! Parse flow: validate length, parse base block, then parse
//! extensions and keep unknown blocks as raw bytes.

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
    #[must_use]
    #[allow(clippy::missing_panics_doc, reason = "The array length was validated.")]
    pub fn base(&self) -> Base<'a> {
        let base: &'a [u8; BLOCK_LEN] = self.raw[..BLOCK_LEN].try_into().unwrap();
        Base::new(base)
    }

    /// Returns an iterator over all available extension blocks.
    #[allow(clippy::missing_panics_doc, reason = "The array length was validated.")]
    pub fn extensions(&self) -> impl Iterator<Item = Extension<'_>> + '_ {
        (0..self.ext_len).map(move |i| {
            let offset = BLOCK_LEN * (i + 1);
            let block: &[u8; BLOCK_LEN] = self.raw[offset..offset + BLOCK_LEN].try_into().unwrap();
            Extension::parse(block)
        })
    }

    /// Validates all (base and extension(s) blocks) EDID data.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let base = self.base();
        let ext_num = base.footer().extension_count() as usize;
        Validation::new()
            .then(base.validate())
            .fail_if(ext_num != self.ext_len, FailureKind::ExtensionCountMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_edid_invalid_length() {
        let short = [0u8; 100];
        assert!(matches!(Edid::parse(&short), Err(ParseError::InvalidLen)));

        let unaligned = [0u8; 200];
        assert!(matches!(
            Edid::parse(&unaligned),
            Err(ParseError::InvalidLen)
        ));
    }

    #[test]
    fn parse_edid_bad_header() {
        let bad = [0u8; 128];
        assert!(matches!(Edid::parse(&bad), Err(ParseError::BadHeader)));
    }

    #[test]
    fn parse_edid_ext_count_too_large() {
        let mut large = [0u8; 128 * 66];
        large[..8].copy_from_slice(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]);
        assert!(matches!(
            Edid::parse(&large),
            Err(ParseError::ExtCountTooLarge)
        ));
    }
}
