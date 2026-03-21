//! EDID parsing for the 128-byte [`Base`] block and [`Extension`]s.
//!
//! Parse flow: validate length, parse base header + checksum, then parse
//! extensions and keep unknown blocks as raw bytes.

use crate::base::Base;
use crate::common::{BLOCK_LEN, ErrorKind, Validation, slice};
use crate::extensions::Extension;
use crate::extensions::cta::Cta;

/// Parsed EDID data with the base block and optional extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid {
    raw: [u8; BLOCK_LEN * (MAX_EXT + 1)],
}

/// Maximum number of extension block.
pub const MAX_EXT: usize = 64;

/// Unrecoverable errors that prevent parsing the EDID data at all.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    InvalidLen,
    BadHeader,
    ExtCountTooLarge,
}

impl Edid {
    /// Parses raw EDID bytes.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError::InvalidLen`] for invalid or mismatched length.
    /// Returns [`ParseError::BadHeader`] for an invalid header pattern.
    /// Returns [`ParseError::ExtCountTooLarge`] if the extension count exceeds the max.
    pub const fn parse(raw: &[u8]) -> Result<Self, ParseError> {
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

        let ext_num = raw[BLOCK_LEN - 2] as usize;
        let max_len = BLOCK_LEN * (MAX_EXT + 1);
        let min_len = BLOCK_LEN * (ext_num + 1);
        if min_len > max_len {
            return Err(ParseError::ExtCountTooLarge);
        }
        if raw.len() < min_len || raw.len() > max_len {
            return Err(ParseError::InvalidLen);
        }

        let mut buf = [0u8; BLOCK_LEN * (MAX_EXT + 1)];
        let mut i = 0;
        while i < raw.len() {
            buf[i] = raw[i];
            i += 1;
        }

        Ok(Self { raw: buf })
    }

    /// Returns the base block.
    #[must_use]
    pub const fn base(&self) -> Base {
        let base_raw: [u8; BLOCK_LEN] = slice(&self.raw, 0);
        Base::new(&base_raw)
    }

    /// Returns the extensions.
    #[must_use]
    pub const fn extensions(&self) -> [Option<Extension>; MAX_EXT] {
        let mut extensions = [None; MAX_EXT];
        let mut i = 0;
        let ext_num = self.base().footer().extension_num() as usize;
        let max = if ext_num < MAX_EXT { ext_num } else { MAX_EXT };
        while i < max {
            let offset = BLOCK_LEN + i * BLOCK_LEN;
            let block: [u8; BLOCK_LEN] = slice(&self.raw, offset);
            extensions[i] = match Cta::parse(&block) {
                Some(cta) => Some(Extension::Cta(cta)),
                None => Some(Extension::Unknown(block)),
            };
            i += 1;
        }
        extensions
    }

    /// Validates the EDID data.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        let base = self.base();
        let base_raw: [u8; BLOCK_LEN] = slice(&self.raw, 0);

        let ext_num = base.footer().extension_num() as usize;
        let extensions = self.extensions();
        let mut ext_len = 0;
        let mut i = 0;
        while i < MAX_EXT {
            if extensions[i].is_some() {
                ext_len += 1;
            }
            i += 1;
        }

        Validation::new()
            .then(base.validate(&base_raw))
            .err_if(ext_num != ext_len, ErrorKind::EdidExtCountMismatch)
    }
}
