//! EDID parsing for the 128-byte [`Base`] block and [`Extension`]s.
//!
//! Parse flow: validate length, parse base header + checksum, then parse
//! extensions and keep unknown blocks as raw bytes.

use crate::base::Base;
use crate::common::{BLOCK_LEN, FailureKind, Validation, slice};
use crate::extensions::Extension;
use crate::extensions::cta::Cta;

/// Parsed EDID data with the base block and optional extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid {
    raw: [u8; BLOCK_LEN * (MAX_EXT + 1)],
    ext_len: usize,
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
    /// Returns [`ParseError::InvalidLen`] for input shorter than one block.
    /// Returns [`ParseError::BadHeader`] for an invalid header pattern.
    /// Returns [`ParseError::ExtCountTooLarge`] for more than the max blocks.
    pub const fn parse(raw: &[u8]) -> Result<Self, ParseError> {
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

        let mut buf = [0u8; MAX_LEN];
        let mut i = 0;
        while i < raw.len() {
            buf[i] = raw[i];
            i += 1;
        }

        let blocks = raw.len() / BLOCK_LEN;
        let ext_len = blocks - 1;
        Ok(Self { raw: buf, ext_len })
    }

    /// Returns the base block.
    #[must_use]
    pub const fn base(&self) -> Base {
        let base_raw: [u8; BLOCK_LEN] = slice(&self.raw, 0);
        Base::new(&base_raw)
    }

    /// Returns extensions up to the reported footer count.
    #[must_use]
    pub const fn extensions(&self) -> [Extension; MAX_EXT] {
        let ext_num = self.base().footer().extension_num() as usize;
        let max = if self.ext_len < ext_num {
            self.ext_len
        } else {
            ext_num
        };
        self.collect_extensions(max)
    }

    /// Returns extensions based on the available blocks.
    #[must_use]
    pub const fn extensions_all(&self) -> [Extension; MAX_EXT] {
        self.collect_extensions(self.ext_len)
    }

    /// Validates the EDID data.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        let base = self.base();
        let ext_num = base.footer().extension_num() as usize;
        Validation::new()
            .then(base.validate())
            .fail_if(ext_num != self.ext_len, FailureKind::EdidExtCountMismatch)
    }

    const fn collect_extensions(&self, max: usize) -> [Extension; MAX_EXT] {
        let mut i = 0;
        let mut extensions = [Extension::Empty; MAX_EXT];
        while i < max {
            let offset = BLOCK_LEN + i * BLOCK_LEN;
            let block: [u8; BLOCK_LEN] = slice(&self.raw, offset);
            extensions[i] = match Cta::parse(&block) {
                Some(cta) => Extension::Cta(cta),
                None => Extension::Unknown(block),
            };
            i += 1;
        }
        extensions
    }
}
