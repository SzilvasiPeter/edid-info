//! Common helpers for EDID parsing.
//!
//! This module provides checksum verification, slice helpers,
//! and validation result types.

/// Length of an EDID block (base or extension) in bytes.
pub const BLOCK_LEN: usize = 128;

/// Length of an descriptor (detailed timing or monitor) in bytes.
pub const DESC_LEN: usize = 18;

/// EDID version (major and minor).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Error variants for EDID validation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FailureKind {
    // --- Base Edid ---
    /// Invalid checksum for the base block.
    BaseChecksum = 0,
    /// Extension count in header does not match parsed blocks.
    EdidExtCountMismatch = 1,

    // --- Base Header ---
    /// Manufacturer ID contains invalid bits (not uppercase letters).
    HeaderMfrInvalidBits = 2,
    /// Week value is invalid (greater than 54 and not 0xFF).
    HeaderWeekInvalid = 3,
    /// EDID major version is zero.
    HeaderMajorInvalid = 4,

    // --- Descriptor ---
    /// Pixel clock is zero.
    TimingPixelClock = 5,

    // --- Extension CTA ---
    /// Invalid checksum for a CTA extension block.
    CtaChecksum = 6,
}

impl FailureKind {
    /// Returns the human-readable error message.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::BaseChecksum => "Invalid base checksum",
            Self::CtaChecksum => "Invalid CTA checksum",
            Self::EdidExtCountMismatch => "Extension count does not match parsed blocks",
            Self::HeaderMfrInvalidBits => "Invalid manufacturer ID bits",
            Self::HeaderWeekInvalid => "Invalid week value",
            Self::HeaderMajorInvalid => "Invalid EDID major version",
            Self::TimingPixelClock => "Invalid pixel clock",
        }
    }
}

/// Warning variants for EDID validation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WarningKind {
    // --- Base Header ---
    /// Manufacturer ID reserved bit (bit 15) is set.
    HeaderMfrReservedSet = 0,
    /// Product code is zero.
    HeaderProductInvalid = 1,
    /// Serial number is zero.
    HeaderSerialInvalid = 2,
    /// EDID version is not 1.4.
    HeaderVersionDeprecated = 3,
}

impl WarningKind {
    /// Returns the human-readable warning message.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::HeaderMfrReservedSet => "Manufacturer ID reserved bit (bit 15) is set",
            Self::HeaderProductInvalid => "Invalid product code",
            Self::HeaderSerialInvalid => "Invalid serial number",
            Self::HeaderVersionDeprecated => "Deprecated EDID version",
        }
    }
}

/// Validation result with errors and warnings represented as bitfields.
/// Error and warning enums must each fit within 64 variants.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Validation {
    /// Fatal errors bitfield.
    pub errors: u64,
    /// Non-fatal warnings bitfield.
    pub warnings: u64,
}

impl Validation {
    /// Create a new empty validation result.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            errors: 0,
            warnings: 0,
        }
    }

    /// Check if validation passed (no errors).
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.errors == 0
    }

    /// Merge another validation result into this one.
    #[must_use]
    pub const fn then(self, other: Self) -> Self {
        Self {
            errors: self.errors | other.errors,
            warnings: self.warnings | other.warnings,
        }
    }

    /// Add a failure if condition is true.
    #[must_use]
    pub const fn fail_if(mut self, cond: bool, kind: FailureKind) -> Self {
        if cond {
            self.errors |= 1 << (kind as u8);
        }
        self
    }

    /// Add a warning if condition is true.
    #[must_use]
    pub const fn warn_if(mut self, cond: bool, kind: WarningKind) -> Self {
        if cond {
            self.warnings |= 1 << (kind as u8);
        }
        self
    }
}

/// Verifies the checksum of an EDID block.
///
/// Returns `true` if the sum of all bytes in the block equals zero (with u8 wrapping).
pub(crate) const fn checksum_ok(raw: &[u8; BLOCK_LEN]) -> bool {
    let mut sum = 0u8;
    let mut i = 0;
    while i < BLOCK_LEN {
        sum = sum.wrapping_add(raw[i]);
        i += 1;
    }
    sum == 0
}

// const fn slice_unchecked<const N: usize>(raw: &[u8], off: usize) -> [u8; N] {
//     let mut out = [0u8; N];
//     let mut i = 0;
//     while i < N {
//         out[i] = raw[off + i];
//         i += 1;
//     }
//     out
// }

// TODO: Check wheter making the slicing const makes any speed difference
pub(crate) fn slice<const N: usize, const M: usize>(raw: &[u8; M], off: usize) -> [u8; N] {
    // assert!(off + N <= M);
    // slice_unchecked(raw, off);
    let mut out = [0u8; N];
    out.copy_from_slice(&raw[off..off + N]);
    out
}
