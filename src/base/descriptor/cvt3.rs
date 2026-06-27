//! CVT 3-Byte Timing Codes descriptor (tag 0xF8).
//!
//! Defines up to 4 video timing modes using 3-byte CVT codes.
//! Each entry encodes the addressable vertical line count, aspect ratio, preferred refresh rate, and supported refresh rates.
//!
//! # Descriptor Layout
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
//! Horizontal pixels are computed as: `H = 8 × trunc((V × aspect.width / aspect.height) / 8)`
//!
//! # Examples
//!
//! ```rust
//! use edid_info::base::descriptor::cvt3::Rate;
//! use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};
//! use edid_info::common::{AspectRatio, DESC_LEN};
//!
//! // Only priority 1 is present; priorities 2-4 are all-zero (absent).
//! let mut raw = [0u8; DESC_LEN];
//! raw[3] = 0xF8; // CVT3 tag
//! raw[5] = 0x01; // Version
//! raw[6] = 0x00; // Addressable lines LSB (v_lines = (0 + 1) * 2 = 2)
//! raw[7] = 0x00; // Lines MSB=0, aspect 4:3=00, reserved=00
//! raw[8] = 0x11; // Preferred 50 Hz, Hz50 + reduced 60 Hz supported
//!
//! let monitor = Monitor::new(&raw, false);
//! if let DisplayDescriptor::Cvt3(cvt3) = monitor.descriptor() {
//!     let p1 = cvt3.priority1().unwrap();
//!     assert_eq!(p1.vertical_lines(), 2);
//!     assert_eq!(p1.horizontal_pixels(), 0);
//!     assert_eq!(p1.aspect_ratio(), AspectRatio::new(4, 3));
//!     assert_eq!(p1.preferred_rate(), Rate::Hz50);
//!     assert!(p1.rates().eq([Rate::Hz50, Rate::Hz60]));
//!     assert!(p1.blanking().standard);
//!     assert!(p1.blanking().reduced);
//!
//!     assert!(cvt3.priority2().is_none());
//!     assert!(cvt3.priority3().is_none());
//!     assert!(cvt3.priority4().is_none());
//!     assert!(cvt3.validate().is_valid());
//! } else {
//!     panic!("expected Cvt3 descriptor");
//! }
//! ```

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

    /// Horizontal pixels, computed as `trunc((v_lines × aspect) / 8) * 8`.
    #[must_use]
    pub const fn horizontal_pixels(&self) -> u16 {
        let v = self.vertical_lines();
        let r = self.aspect_ratio();
        let h = (v / r.height()) * r.width() + (v % r.height()) * r.width() / r.height();
        (h / 8) * 8
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

    /// Highest priority entry.
    ///
    /// Returns `None` when the three-byte encoding is all-zero (entry absent).
    #[must_use]
    pub const fn priority1(&self) -> Option<Entry> {
        let raw = [self.raw[1], self.raw[2], self.raw[3]];
        if raw[0] == 0 && raw[1] == 0 && raw[2] == 0 {
            None
        } else {
            Some(Entry { raw })
        }
    }

    /// Second priority entry.
    ///
    /// Returns `None` when the three-byte encoding is all-zero (entry absent).
    #[must_use]
    pub const fn priority2(&self) -> Option<Entry> {
        let raw = [self.raw[4], self.raw[5], self.raw[6]];
        if raw[0] == 0 && raw[1] == 0 && raw[2] == 0 {
            None
        } else {
            Some(Entry { raw })
        }
    }

    /// Third priority entry.
    ///
    /// Returns `None` when the three-byte encoding is all-zero (entry absent).
    #[must_use]
    pub const fn priority3(&self) -> Option<Entry> {
        let raw = [self.raw[7], self.raw[8], self.raw[9]];
        if raw[0] == 0 && raw[1] == 0 && raw[2] == 0 {
            None
        } else {
            Some(Entry { raw })
        }
    }

    /// Lowest priority entry.
    ///
    /// Returns `None` when the three-byte encoding is all-zero (entry absent).
    #[must_use]
    pub const fn priority4(&self) -> Option<Entry> {
        let raw = [self.raw[10], self.raw[11], self.raw[12]];
        if raw[0] == 0 && raw[1] == 0 && raw[2] == 0 {
            None
        } else {
            Some(Entry { raw })
        }
    }

    /// Validates the CVT 3-byte descriptor.
    ///
    /// **Failures**: preferred rate not listed in supported rates for a non-zero entry.
    /// **Warnings**: version is not `0x01`, or reserved bits are non-zero.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let pref_not_supported = [
            self.priority1(),
            self.priority2(),
            self.priority3(),
            self.priority4(),
        ]
        .iter()
        .flatten()
        .any(|e| !e.rates().any(|r| r == e.preferred_rate()));

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

#[cfg(test)]
mod tests {
    use super::Rate;
    use crate::base::descriptor::monitor::{DisplayDescriptor, Monitor};
    use crate::common::{AspectRatio, DESC_LEN};

    #[test]
    fn examples_docstring() {
        // Only priority 1 is present; priorities 2-4 are all-zero (absent).
        let mut raw = [0u8; DESC_LEN];
        raw[3] = 0xF8; // CVT3 tag
        raw[5] = 0x01; // Version
        raw[6] = 0x00; // Addressable lines LSB (v_lines = (0 + 1) * 2 = 2)
        raw[7] = 0x00; // Lines MSB=0, aspect 4:3=00, reserved=00
        raw[8] = 0x11; // Preferred 50 Hz, Hz50 + reduced 60 Hz supported

        let monitor = Monitor::new(&raw, false);
        if let DisplayDescriptor::Cvt3(cvt3) = monitor.descriptor() {
            let p1 = cvt3.priority1().unwrap();
            assert_eq!(p1.vertical_lines(), 2);
            assert_eq!(p1.horizontal_pixels(), 0);
            assert_eq!(p1.aspect_ratio(), AspectRatio::new(4, 3));
            assert_eq!(p1.preferred_rate(), Rate::Hz50);
            assert!(p1.rates().eq([Rate::Hz50, Rate::Hz60]));
            assert!(p1.blanking().standard);
            assert!(p1.blanking().reduced);

            assert!(cvt3.priority2().is_none());
            assert!(cvt3.priority3().is_none());
            assert!(cvt3.priority4().is_none());
            assert!(cvt3.validate().is_valid());
        }
    }

    #[test]
    fn parse_cvt3_synthetic() {
        let mut raw = [0u8; 18];
        raw[3] = 0xF8;
        raw[5] = 0x01;
        // Priority 1: 4:3, 50 Hz, Hz50 supported
        raw[6] = 0x00;
        raw[7] = 0x00;
        raw[8] = 0b0001_0000;
        // Priority 2: 16:9, 60 Hz, Hz50+60 supported
        raw[9] = 0x10;
        raw[10] = 0x04;
        raw[11] = 0x38;
        // Priority 3: 16:10, 75 Hz, Hz75 supported + reduced blanking
        raw[12] = 0x20;
        raw[13] = 0x08;
        raw[14] = 0x45;
        // Priority 4: 15:9, 85 Hz, Hz85 supported
        raw[15] = 0x30;
        raw[16] = 0x0C;
        raw[17] = 0x62;

        let monitor = Monitor::new(&raw, false);
        if let DisplayDescriptor::Cvt3(cvt3) = monitor.descriptor() {
            let p1 = cvt3.priority1().unwrap();
            assert_eq!(p1.vertical_lines(), 2);
            assert_eq!(p1.horizontal_pixels(), 0);
            assert_eq!(p1.aspect_ratio(), AspectRatio::new(4, 3));
            assert_eq!(p1.preferred_rate(), Rate::Hz50);
            assert!(p1.rates().eq([Rate::Hz50]));
            assert!(p1.blanking().standard);
            assert!(!p1.blanking().reduced);

            let p2 = cvt3.priority2().unwrap();
            assert_eq!(p2.vertical_lines(), 34);
            assert_eq!(p2.horizontal_pixels(), 56);
            assert_eq!(p2.aspect_ratio(), AspectRatio::new(16, 9));
            assert_eq!(p2.preferred_rate(), Rate::Hz60);
            assert!(p2.rates().eq([Rate::Hz50, Rate::Hz60]));
            assert!(p2.blanking().standard);
            assert!(!p2.blanking().reduced);

            let p3 = cvt3.priority3().unwrap();
            assert_eq!(p3.vertical_lines(), 66);
            assert_eq!(p3.horizontal_pixels(), 104);
            assert_eq!(p3.aspect_ratio(), AspectRatio::new(16, 10));
            assert_eq!(p3.preferred_rate(), Rate::Hz75);
            assert!(p3.rates().eq([Rate::Hz75, Rate::Hz60]));
            assert!(p3.blanking().standard);
            assert!(p3.blanking().reduced);

            let p4 = cvt3.priority4().unwrap();
            assert_eq!(p4.vertical_lines(), 98);
            assert_eq!(p4.horizontal_pixels(), 160);
            assert_eq!(p4.aspect_ratio(), AspectRatio::new(15, 9));
            assert_eq!(p4.preferred_rate(), Rate::Hz85);
            assert!(p4.rates().eq([Rate::Hz85]));
            assert!(p4.blanking().standard);
            assert!(!p4.blanking().reduced);
        }
    }
}
