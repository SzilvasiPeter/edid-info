//! CVT 3-Byte Timing Codes descriptor (tag 0xF8).
//!
//! Defines up to 4 video timing modes using 3-byte CVT codes.
//! Each entry encodes the addressable vertical line count, aspect ratio, preferred refresh rate, and supported refresh rates.
//!
//! # Descriptor Layout (18 bytes)
//!
//! | Byte | Content |
//! |------|---------|
//! | 0–4  | `0x0000F800` (tag header) |
//! | 5    | Version (`0x01`) |
//! | 6–8  | Priority 1 (highest) |
//! | 9–11 | Priority 2 |
//! | 12–14| Priority 3 |
//! | 15–17| Priority 4 (lowest) |
//!
//! # 3-Byte Entry Layout
//!
//! | Byte | Bits  | Field |
//! |------|-------|-------|
//! | 0    | 7–0   | Addressable lines LSBs (8 of 12 bits) |
//! | 1    | 7–4   | Addressable lines MSBs (4 of 12 bits) |
//! | 1    | 3–2   | Aspect ratio |
//! | 1    | 1–0   | Reserved (00) |
//! | 2    | 7     | Reserved (0) |
//! | 2    | 6–5   | Preferred vertical rate |
//! | 2    | 4     | 50 Hz standard blanking supported |
//! | 2    | 3     | 60 Hz standard blanking supported |
//! | 2    | 2     | 75 Hz standard blanking supported |
//! | 2    | 1     | 85 Hz standard blanking supported |
//! | 2    | 0     | 60 Hz reduced blanking supported |
//!
//! The entry stores addressable lines value `a = (v_lines / 2) - 1`.
//! Horizontal pixels are computed as:
//! `H = 8 × trunc((V × aspect.width / aspect.height) / 8)`

use crate::common::{AspectRatio, Blanking, FailureKind, Validation, WarningKind};

/// Vertical refresh rate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rate {
    Hz50,
    Hz60,
    Hz75,
    Hz85,
}

/// A single CVT 3-byte timing entry.
///
/// Lazily computes fields from the raw 3-byte encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entry {
    raw: [u8; 3],
}

impl Entry {
    /// Addressable vertical lines.
    #[must_use]
    pub const fn vertical_lines(&self) -> u16 {
        let addr_lines = (self.raw[0] as u16) | ((self.raw[1] as u16 >> 4) & 0x0F) << 8;
        (addr_lines + 1) * 2
    }

    /// Aspect ratio.
    #[must_use]
    pub const fn aspect_ratio(&self) -> AspectRatio {
        match (self.raw[1] >> 2) & 0b11 {
            0b00 => AspectRatio::new(4, 3),
            0b01 => AspectRatio::new(16, 9),
            0b10 => AspectRatio::new(16, 10),
            _ => AspectRatio::new(15, 9),
        }
    }

    /// Preferred refresh rate.
    #[must_use]
    pub const fn preferred_rate(&self) -> Rate {
        match (self.raw[2] >> 5) & 0b11 {
            0b00 => Rate::Hz50,
            0b01 => Rate::Hz60,
            0b10 => Rate::Hz75,
            _ => Rate::Hz85,
        }
    }

    /// Supported refresh rates (standard blanking).
    pub fn rates(&self) -> impl Iterator<Item = Rate> {
        let bits = self.raw[2];
        [
            (0b0001_0000, Rate::Hz50),
            (0b0000_1000, Rate::Hz60),
            (0b0000_0100, Rate::Hz75),
            (0b0000_0010, Rate::Hz85),
            (0b0000_0001, Rate::Hz60), // with reduced blanking
        ]
        .into_iter()
        .filter_map(move |(mask, rate)| (bits & mask != 0).then_some(rate))
    }

    /// Blanking support.
    #[must_use]
    pub const fn blanking(&self) -> Blanking {
        Blanking {
            standard: self.raw[2] & 0b0001_1110 != 0,
            reduced: self.raw[2] & 0b0000_0001 != 0,
        }
    }

    /// Horizontal pixels, computed as `trunc((v_lines × aspect) / 8) * 8`.
    #[must_use]
    pub const fn horizontal_pixels(&self) -> u16 {
        let v = self.vertical_lines();
        let r = self.aspect_ratio();
        let h = (v / r.height()) * r.width() + (v % r.height()) * r.width() / r.height();
        (h / 8) * 8
    }
}

/// CVT 3-Byte Timing Codes (tag 0xF8).
///
/// Stores the raw 13-byte descriptor payload and provides access to up to 4 prioritized timing entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cvt3 {
    raw: [u8; 13],
}

impl Cvt3 {
    pub(super) const fn new(raw: &[u8; 13]) -> Self {
        Self { raw: *raw }
    }

    /// Highest priority entry (descriptor bytes 6–8).
    #[must_use]
    pub const fn priority1(&self) -> Entry {
        let raw = [self.raw[1], self.raw[2], self.raw[3]];
        Entry { raw }
    }

    /// Second priority entry (descriptor bytes 9–11).
    #[must_use]
    pub const fn priority2(&self) -> Entry {
        let raw = [self.raw[4], self.raw[5], self.raw[6]];
        Entry { raw }
    }

    /// Third priority entry (descriptor bytes 12–14).
    #[must_use]
    pub const fn priority3(&self) -> Entry {
        let raw = [self.raw[7], self.raw[8], self.raw[9]];
        Entry { raw }
    }

    /// Lowest priority entry (descriptor bytes 15–17).
    #[must_use]
    pub const fn priority4(&self) -> Entry {
        let raw = [self.raw[10], self.raw[11], self.raw[12]];
        Entry { raw }
    }

    /// Validates the CVT 3-byte descriptor.
    ///
    /// **Failures**: preferred rate not listed in supported rates for a non-zero entry.
    /// **Warnings**: version is not `0x01`, or reserved bits are non-zero.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let entries: [_; 4] = [
            self.priority1(),
            self.priority2(),
            self.priority3(),
            self.priority4(),
        ];
        let pref_not_supported = entries
            .iter()
            .any(|e| e.raw != [0; 3] && !e.rates().any(|r| r == e.preferred_rate()));

        let bad_reserved = (0..4).any(|i| {
            let off = 1 + i * 3;
            (self.raw[off + 1] & 0b11) != 0 || (self.raw[off + 2] & 0b1000_0000) != 0
        });

        Validation::new()
            .fail_if(pref_not_supported, FailureKind::Cvt3PreferredRateMismatch)
            .warn_if(self.raw[0] != 0x01, WarningKind::Cvt3VersionReserved)
            .warn_if(bad_reserved, WarningKind::Cvt3ReservedBits)
    }
}
