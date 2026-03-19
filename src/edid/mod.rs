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
pub(crate) const fn slice<const N: usize, const M: usize>(raw: &[u8; M], off: usize) -> [u8; N] {
    let mut out = [0u8; N];
    let mut i = 0;
    while i < N {
        out[i] = raw[off + i];
        i += 1;
    }
    out
}

/// Parsed EDID data with the base block and optional extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid {
    base: base::BaseEdid,
    extensions: Vec<Extension>,
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

// TODO: unify the APIs: new (fail-free byte encoding), getters (human-readable where applicable), validate (errors and warnings)
impl Edid {
    /// Parses raw EDID bytes.
    #[must_use]
    pub fn parse(raw: &[u8]) -> Option<Self> {
        if raw.len() < BLOCK_LEN {
            return None;
        }

        if !raw.len().is_multiple_of(BLOCK_LEN) {
            return None;
        }

        let base_raw: [u8; BLOCK_LEN] = std::array::from_fn(|i| raw[i]);
        let base = base::BaseEdid::parse(&base_raw)?;

        let ext_num = base.footer().extension_num() as usize;
        let blocks_available = raw.len() / BLOCK_LEN - 1;
        let extensions = raw[BLOCK_LEN..]
            .chunks_exact(BLOCK_LEN)
            .take(ext_num.min(blocks_available))
            .map(|chunk| {
                let block: [u8; BLOCK_LEN] = std::array::from_fn(|i| chunk[i]);
                cta::Cta::parse(&block).map_or(Extension::Unknown(block), Extension::Cta)
            })
            .collect();

        Some(Self { base, extensions })
    }

    /// Returns the base block.
    #[must_use]
    pub const fn base(&self) -> &base::BaseEdid {
        &self.base
    }

    /// Returns the extensions.
    #[must_use]
    pub fn extensions(&self) -> &[Extension] {
        &self.extensions
    }

    #[must_use]
    pub fn validate(&self) -> Validation {
        let ext_num = self.base.footer().extension_num() as usize;
        Validation::new().then(self.base.validate()).err_if(
            ext_num != self.extensions.len(),
            "Extension count differs from extension length",
        )
    }
}
