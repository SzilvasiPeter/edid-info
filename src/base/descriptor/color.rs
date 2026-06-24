//! White Point Data Descriptor (tag `FBh`).
//!
//! Contains up to two white point index entries with chromaticity coordinates and gamma values.
//!
//! # Structure
//!
//! Each white point entry occupies 5 bytes in the descriptor payload (bytes 5–9 for the first entry, bytes 10–14 for the second):
//!
//! | Offset | Field |
//! |--------|-------|
//! | +0     | Index (0 = unused) |
//! | +1     | Combined LSB nibble: bits 3–2 = x low, bits 1–0 = y low |
//! | +2     | White point x upper 8 bits (bit 9 → bit 2) |
//! | +3     | White point y upper 8 bits (bit 9 → bit 2) |
//! | +4     | Gamma: `stored = (gamma × 100) - 100`, `0xFF` = undefined |
//!
//! Chromaticity coordinates are 10-bit binary fractions where bit 9 = 2^-1 = 0.5 and bit 0 = 2^-10 ≈ 0.00098.
//! Gamma is decoded as `gamma_raw / 100.0 + 1.0`, valid range 1.00–3.54.
//!
//! Trailing bytes 15–17 of the descriptor must contain 0x0A (LF) followed by 0x20 0x20 (Space Space).

use crate::base::chroma::Coord;
use crate::common::{FailureKind, Validation, WarningKind};

/// A single white point entry.
///
/// Stores the raw 5-byte payload and lazily computes coordinates and gamma on access.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    raw: [u8; 5],
}

impl Point {
    /// Chromaticity coordinate of this white point.
    #[must_use]
    pub const fn coord(&self) -> Coord {
        let lsb = self.raw[1];
        let x = ((self.raw[2] as u16) << 2) | ((lsb >> 2) & 0b11) as u16;
        let y = ((self.raw[3] as u16) << 2) | (lsb & 0b11) as u16;
        Coord { x, y }
    }

    /// Decoded gamma value.
    ///
    /// Returns `None` when `gamma_raw` is `0xFF` (undefined).
    /// Otherwise returns `gamma_raw / 100.0 + 1.0`, range 1.00–3.54.
    #[must_use]
    pub fn gamma(&self) -> Option<f32> {
        if self.raw[4] == 0xFF {
            None
        } else {
            Some(f32::from(self.raw[4]) / 100.0 + 1.0)
        }
    }
}

/// Color Point Descriptor (tag 0xFB).
///
/// Holds the 13-byte payload (bytes 5–17 of the 18-byte descriptor)
/// and provides access to up to two white point entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorPoint {
    raw: [u8; 13],
}

impl ColorPoint {
    #[must_use]
    pub(super) const fn new(raw: &[u8; 13]) -> Self {
        Self { raw: *raw }
    }

    /// First white point entry (bytes 5–9 of the descriptor).
    ///
    /// Returns `None` when the index is zero (entry absent).
    #[must_use]
    pub const fn first(&self) -> Option<Point> {
        if self.raw[0] == 0 {
            return None;
        }
        Some(Point {
            raw: [
                self.raw[0],
                self.raw[1],
                self.raw[2],
                self.raw[3],
                self.raw[4],
            ],
        })
    }

    /// Second white point entry (bytes 10–14 of the descriptor).
    ///
    /// Per spec, the second index must be in range `02h–FFh`.
    /// Returns `None` when the index is `0x00` (absent) or `0x01` (invalid for a second entry).
    #[must_use]
    pub const fn second(&self) -> Option<Point> {
        let idx = self.raw[5];
        if idx == 0 || idx == 1 {
            return None;
        }
        Some(Point {
            raw: [
                self.raw[5],
                self.raw[6],
                self.raw[7],
                self.raw[8],
                self.raw[9],
            ],
        })
    }

    /// Validates the color point descriptor.
    ///
    /// **Failures**: reserved bits (bits 7–4 of the combined LSB byte)
    /// are non-zero for any present entry.
    ///
    /// **Warnings**: trailing bytes deviate from the expected
    /// `0x0A 0x20 0x20` (LF, Space, Space).
    #[must_use]
    pub const fn validate(&self) -> Validation {
        let raw = self.raw;
        let first_lsb = raw[1];
        let second_lsb = raw[6];

        let reserved_fail = FailureKind::ColorPointReservedBits;
        Validation::new()
            .fail_if(raw[0] != 0 && (first_lsb & 0xF0) != 0, reserved_fail)
            .fail_if(raw[5] >= 2 && (second_lsb & 0xF0) != 0, reserved_fail)
            .warn_if(raw[10] != 0x0A, WarningKind::ColorPointExpectedLineFeed)
            .warn_if(raw[11] != 0x20, WarningKind::ColorPointExpectedSpaces)
            .warn_if(raw[12] != 0x20, WarningKind::ColorPointExpectedSpaces)
    }
}
