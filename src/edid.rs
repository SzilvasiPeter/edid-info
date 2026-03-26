//! EDID parsing for the 128-byte [`Base`] block and [`Extension`]s.
//!
//! Parse flow: validate length, parse base block, then parse
//! extensions and keep unknown blocks as raw bytes.

use crate::base::Base;
use crate::common::{BLOCK_LEN, FailureKind, Validation};
use crate::extensions::Extension;
use crate::extensions::cta::Cta;

/// Parsed EDID data with the base block and optional extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid<'a> {
    raw: &'a [u8],
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

        if raw.len() > BLOCK_LEN * (MAX_EXT + 1) {
            return Err(ParseError::ExtCountTooLarge);
        }

        if raw[0..8] != [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00] {
            return Err(ParseError::BadHeader);
        }

        let ext_len = (raw.len() / BLOCK_LEN) - 1;
        Ok(Self { raw, ext_len })
    }

    /// Returns the base block.
    ///
    /// # Panics
    ///
    /// Panics if the internal buffer is shorter than 128 bytes. This should
    /// never happen because `Edid::parse` validates the length.
    #[must_use]
    pub fn base(&self) -> Base<'_> {
        let base_raw: &[u8; BLOCK_LEN] = self.raw[..BLOCK_LEN].try_into().unwrap();
        Base::new(base_raw)
    }

    /// Returns extensions up to the reported footer count.
    #[must_use]
    pub fn extensions(&self) -> [Extension; MAX_EXT] {
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
    pub fn extensions_all(&self) -> [Extension; MAX_EXT] {
        self.collect_extensions(self.ext_len)
    }

    /// Validates the full (base and extension(s) blocks) EDID data.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let base = self.base();
        let ext_num = base.footer().extension_num() as usize;
        Validation::new()
            .then(base.validate())
            .fail_if(ext_num != self.ext_len, FailureKind::BaseExtCountMismatch)
    }

    fn collect_extensions(&self, max: usize) -> [Extension; MAX_EXT] {
        let mut out = [Extension::Empty; MAX_EXT];
        for (i, ext) in out.iter_mut().enumerate().take(max) {
            let off = BLOCK_LEN * (i + 1);
            let block: [u8; BLOCK_LEN] = self.raw[off..off + BLOCK_LEN].try_into().unwrap();
            *ext = Cta::parse(&block).map_or(Extension::Unknown(block), Extension::Cta);
        }
        out
    }
}
