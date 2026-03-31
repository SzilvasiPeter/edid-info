//! EDID parsing for the 128-byte [`Base`] block and [`Extension`]s.

extern crate alloc;

use alloc::vec::Vec;

use edid_info::base::Base;
use edid_info::common::{BLOCK_LEN, FailureKind, Validation};
use edid_info::extensions::Extension;
use edid_info::extensions::cta::Cta;

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
pub struct Edid {
    raw: Vec<u8>,
    ext_len: usize,
}

#[allow(dead_code)]
impl Edid {
    /// Parses raw EDID bytes.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError::InvalidLen`] for input shorter than one block.
    /// Returns [`ParseError::BadHeader`] for an invalid header pattern.
    /// Returns [`ParseError::ExtCountTooLarge`] for more than the max blocks.
    pub fn parse(raw: &[u8]) -> Result<Self, ParseError> {
        const MAX_LEN: usize = BLOCK_LEN * (MAX_EXT + 1);

        if raw.len() < BLOCK_LEN || !raw.len().is_multiple_of(BLOCK_LEN) {
            return Err(ParseError::InvalidLen);
        }

        if raw[0] != 0x00
            || raw[1] != 0xFF
            || raw[2] != 0xFF
            || raw[3] != 0xFF
            || raw[4] != 0xFF
            || raw[5] != 0xFF
            || raw[6] != 0xFF
            || raw[7] != 0x00
        {
            return Err(ParseError::BadHeader);
        }

        if raw.len() > MAX_LEN {
            return Err(ParseError::ExtCountTooLarge);
        }

        let ext_len = (raw.len() / BLOCK_LEN) - 1;
        let raw = Vec::from(raw);
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

    /// Returns all available extension blocks.
    ///
    /// # Panics
    ///
    /// Panics if the internal buffer is shorter than expected.
    /// This should never happen because `Edid::parse` validates the length.
    #[must_use]
    pub fn extensions(&self) -> Vec<Extension> {
        let mut out = Vec::with_capacity(self.ext_len);
        for i in 0..self.ext_len {
            let off = BLOCK_LEN * (i + 1);
            let block: [u8; BLOCK_LEN] = self.raw[off..off + BLOCK_LEN].try_into().unwrap();
            let item = Cta::parse(&block).map_or(Extension::Unknown(block), Extension::Cta);
            out.push(item);
        }
        out
    }

    /// Validates the EDID data.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let base = self.base();
        let ext_num = base.footer().extension_num() as usize;
        Validation::new()
            .then(base.validate())
            .fail_if(ext_num != self.ext_len, FailureKind::BaseExtCountMismatch)
    }
}
