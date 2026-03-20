//! EDID parsing for the 128-byte [`base::BaseEdid`] block and [`Extension`]s.
//!
//! Parse flow: validate length, parse base header + checksum, then parse
//! extensions and keep unknown blocks as raw bytes.

pub mod base;
pub mod basic;
pub mod bits;
pub mod check;
pub mod chroma;
pub mod cta;
pub mod descriptors;
pub mod established;
pub mod footer;
pub mod header;
pub mod monitor_descriptor;
pub mod std1;
pub mod timing_descriptor;

/// Length of an EDID block (base or extension) in bytes.
pub const BLOCK_LEN: usize = 128;

// TODO: Check the linuxhw/EDID for the maximum number of extension blocks
/// Maximum number of extension block.
pub const MAX_EXT: usize = 10;

const fn slice_unchecked<const N: usize>(raw: &[u8], off: usize) -> [u8; N] {
    let mut out = [0u8; N];
    let mut i = 0;
    while i < N {
        out[i] = raw[off + i];
        i += 1;
    }
    out
}

pub(crate) const fn slice<const N: usize, const M: usize>(raw: &[u8; M], off: usize) -> [u8; N] {
    assert!(off + N <= M);
    slice_unchecked(raw, off)
}

pub(crate) const fn slice_raw<const N: usize>(raw: &[u8], off: usize) -> [u8; N] {
    assert!(off + N <= raw.len());
    slice_unchecked(raw, off)
}

/// Parsed EDID data with the base block and optional extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid {
    base: base::BaseEdid,
    extensions: [Option<Extension>; MAX_EXT],
}

/// EDID extension block types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extension {
    /// CEA-861 extension block.
    Cta(cta::Cta),
    /// Unrecognized extension type, stored as raw bytes.
    Unknown([u8; BLOCK_LEN]),
}

/// Unrecoverable errors that prevent parsing the EDID data at all.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The input data is too short (minimum 128 bytes).
    TooShort,
    /// The input data length is not a multiple of 128 bytes.
    InvalidLength,
}

/// Validation result with errors and warnings.
#[derive(Clone, Debug, Default)]
pub struct Validation {
    /// Fatal errors that indicate invalid data.
    pub errors: Vec<String>,
    /// Non-fatal warnings about spec deviations.
    pub warnings: Vec<String>,
}

impl Validation {
    /// Create a new empty validation result.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Check if validation passed (no errors).
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Merge another validation result into this one.
    #[must_use]
    pub fn then(self, other: Self) -> Self {
        let mut errors = self.errors;
        let mut warnings = self.warnings;
        errors.extend(other.errors);
        warnings.extend(other.warnings);
        Self { errors, warnings }
    }

    /// Add an error if condition is true.
    #[must_use]
    pub fn err_if(self, cond: bool, msg: impl Into<String>) -> Self {
        if cond {
            let mut errors = self.errors;
            errors.push(msg.into());
            Self { errors, ..self }
        } else {
            self
        }
    }

    /// Add a warning if condition is true.
    #[must_use]
    pub fn warn_if(self, cond: bool, msg: impl Into<String>) -> Self {
        if cond {
            let mut warnings = self.warnings;
            warnings.push(msg.into());
            Self { warnings, ..self }
        } else {
            self
        }
    }
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
        let Some(base) = base::BaseEdid::parse(&base_raw) else {
            return None;
        };

        let mut extensions = [None; MAX_EXT];
        let mut i = 0;
        let nblock = raw.len() / BLOCK_LEN - 1;
        let max = if nblock < MAX_EXT { nblock } else { MAX_EXT };
        while i < max {
            let offset = BLOCK_LEN + i * BLOCK_LEN;
            let block: [u8; BLOCK_LEN] = slice_raw(raw, offset);
            extensions[i] = match cta::Cta::parse(&block) {
                Some(cta) => Some(Extension::Cta(cta)),
                None => Some(Extension::Unknown(block)),
            };
            i += 1;
        }

        Some(Self { base, extensions })
    }

    /// Returns the base block.
    #[must_use]
    pub const fn base(&self) -> &base::BaseEdid {
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
