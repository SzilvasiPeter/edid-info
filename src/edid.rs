//! EDID parsing for the 128-byte [`base::BaseEdid`] block and [`Extension`]s.
//!
//! Parse flow: validate length, parse base header + checksum, then parse
//! extensions and keep unknown blocks as raw bytes.

use crate::base::BaseEdid;
use crate::common::{BLOCK_LEN, Validation, slice_raw};
use crate::extensions::Extension;
use crate::extensions::cta::Cta;

/// Parsed EDID data with the base block and optional extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid {
    base: BaseEdid,
    extensions: [Option<Extension>; MAX_EXT],
}

/// Maximum number of extension block.
pub const MAX_EXT: usize = 64;

/// Unrecoverable errors that prevent parsing the EDID data at all.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The input data is too short (minimum 128 bytes).
    TooShort,
    /// The input data length is not a multiple of 128 bytes.
    InvalidLength,
}

impl Edid {
    /// Parses raw EDID bytes.
    #[must_use]
    pub const fn parse(raw: &[u8]) -> Option<Self> {
        if raw.len() < BLOCK_LEN {
            return None;
        }

        if !raw.len().is_multiple_of(BLOCK_LEN) {
            return None;
        }

        let base_raw: [u8; BLOCK_LEN] = slice_raw(raw, 0);
        let Some(base) = BaseEdid::parse(&base_raw) else {
            return None;
        };

        let mut extensions = [None; MAX_EXT];
        let mut i = 0;
        let nblock = raw.len() / BLOCK_LEN - 1;
        let max = if nblock < MAX_EXT { nblock } else { MAX_EXT };
        while i < max {
            let offset = BLOCK_LEN + i * BLOCK_LEN;
            let block: [u8; BLOCK_LEN] = slice_raw(raw, offset);
            extensions[i] = match Cta::parse(&block) {
                Some(cta) => Some(Extension::Cta(cta)),
                None => Some(Extension::Unknown(block)),
            };
            i += 1;
        }

        Some(Self { base, extensions })
    }

    /// Returns the base block.
    #[must_use]
    pub const fn base(&self) -> &BaseEdid {
        &self.base
    }

    /// Returns the extensions.
    #[must_use]
    pub const fn extensions(&self) -> &[Option<Extension>; MAX_EXT] {
        &self.extensions
    }

    /// Validates the EDID data.
    /// Requires the original raw bytes for checksum validation.
    #[must_use]
    pub fn validate(&self, base_raw: &[u8; BLOCK_LEN]) -> Validation {
        let ext_num = self.base.footer().extension_num() as usize;
        let ext_len = self.extensions.iter().filter(|e| e.is_some()).count();
        Validation::new().then(self.base.validate(base_raw)).err_if(
            ext_num != ext_len,
            "Extension count does not match parsed blocks",
        )
    }
}
