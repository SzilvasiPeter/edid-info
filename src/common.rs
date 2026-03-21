//! Common helpers for EDID parsing.
//!
//! This module provides checksum verification, slice helpers,
//! and validation result types.

/// Length of an EDID block (base or extension) in bytes.
pub const BLOCK_LEN: usize = 128;

/// Length of an descriptor (detailed timing or monitor) in bytes.
pub const DESC_LEN: usize = 18;

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

/// Verifies the checksum of an EDID block.
///
/// Returns `true` if the sum of all bytes in the block equals zero (with u8 wrapping).
#[must_use]
pub const fn checksum_ok(raw: &[u8; BLOCK_LEN]) -> bool {
    let mut sum = 0u8;
    let mut i = 0;
    while i < BLOCK_LEN {
        sum = sum.wrapping_add(raw[i]);
        i += 1;
    }
    sum == 0
}

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
