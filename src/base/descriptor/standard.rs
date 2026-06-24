//! Standard Timing Identifier descriptor (tag 0xFA).
//!
//! Provides up to 6 additional standard timings beyond the 8 in the base block.
//! Uses the same 2-byte encoding as [`crate::base::standard::StdTimings`].
//!
//! # Descriptor Layout (18 bytes)
//!
//! | Byte | Content |
//! |------|---------|
//! | 0–1  | `0x0000` (monitor descriptor marker) |
//! | 2    | `0x00` (reserved) |
//! | 3    | `0xFA` (tag) |
//! | 4    | `0x00` (reserved) |
//! | 5–16 | 6 standard timing entries, 2 bytes each |
//! | 17   | `0x0A` (line feed) |
//!
//! The 6 entries are labeled *Standard Timing Identification 9–14* in the spec,
//! continuing sequentially from the 8 entries in the base block.

use crate::base::standard::{StandardTiming, parse_std};
use crate::common::{Validation, WarningKind};

/// Additional standard timings (tag 0xFA).
///
/// Holds the 13-byte descriptor payload and provides iteration over the 6 timing entries and a trailing byte.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdTimings {
    raw: [u8; 13],
    legacy: bool,
}

impl StdTimings {
    pub(super) const fn new(raw: &[u8; 13], legacy: bool) -> Self {
        Self { raw: *raw, legacy }
    }

    /// Returns an iterator over the defined standard timings.
    ///
    /// Skips unused entries (encoded as `0x01 0x01`).
    pub fn iter(&self) -> impl Iterator<Item = StandardTiming> {
        self.raw[..12]
            .chunks_exact(2)
            .filter_map(move |c| parse_std(c[0], c[1], self.legacy))
    }

    /// Validates the standard timing descriptor.
    ///
    /// **Warnings**: trailer byte (byte 17) is not `0x0A` (LF).
    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new().warn_if(self.raw[12] != 0x0A, WarningKind::StdTimingExpectedLineFeed)
    }
}
