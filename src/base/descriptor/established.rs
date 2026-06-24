//! Established Timings III descriptor (tag 0xF7).
//!
//! A bitmap of supported additional standard timings defined in the
//! VESA Monitor Timing Standard but not included in Established Timings I or II.
//!
//! # Descriptor Layout (18 bytes)
//!
//! | Byte | Content |
//! |------|---------|
//! | 0–4  | `0x0000F700` (tag header) |
//! | 5    | Version (`0x0A`) |
//! | 6    | Bitmap byte 0 |
//! | 7    | Bitmap byte 1 |
//! | 8    | Bitmap byte 2 |
//! | 9    | Bitmap byte 3 |
//! | 10   | Bitmap byte 4 |
//! | 11   | Bitmap byte 5 (bits 7–4: timings; bits 3–0: reserved) |
//! | 12–17| Reserved (`0x00`) |
//!
//! Each bit corresponds to a DMT timing entry. Support is indicated by a `1`.

use crate::base::dmt::Dmt;
use crate::base::established::flag_dmt;
use crate::common::{Validation, WarningKind};

/// Established Timings III (tag 0xF7).
///
/// Stores the raw 13-byte descriptor payload and provides access to supported DMT timings via [`iter`](Self::iter).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstTimings {
    raw: [u8; 13],
}

impl EstTimings {
    pub(super) const fn new(raw: &[u8; 13]) -> Self {
        Self { raw: *raw }
    }

    /// Supported established timings.
    pub fn iter(&self) -> impl Iterator<Item = Dmt> {
        let b0 = self.raw[1];
        let b1 = self.raw[2];
        let b2 = self.raw[3];
        let b3 = self.raw[4];
        let b4 = self.raw[5];
        let b5 = self.raw[6];
        [
            flag_dmt(b0, 0x80, 0x01),
            flag_dmt(b0, 0x40, 0x02),
            flag_dmt(b0, 0x20, 0x03),
            flag_dmt(b0, 0x10, 0x07),
            flag_dmt(b0, 0x08, 0x0E),
            flag_dmt(b0, 0x04, 0x0C),
            flag_dmt(b0, 0x02, 0x13),
            flag_dmt(b0, 0x01, 0x15),
            flag_dmt(b1, 0x80, 0x16),
            flag_dmt(b1, 0x40, 0x17),
            flag_dmt(b1, 0x20, 0x18),
            flag_dmt(b1, 0x10, 0x19),
            flag_dmt(b1, 0x08, 0x20),
            flag_dmt(b1, 0x04, 0x21),
            flag_dmt(b1, 0x02, 0x23),
            flag_dmt(b1, 0x01, 0x25),
            flag_dmt(b2, 0x80, 0x27),
            flag_dmt(b2, 0x40, 0x2E),
            flag_dmt(b2, 0x20, 0x2F),
            flag_dmt(b2, 0x10, 0x30),
            flag_dmt(b2, 0x08, 0x31),
            flag_dmt(b2, 0x04, 0x29),
            flag_dmt(b2, 0x02, 0x2A),
            flag_dmt(b2, 0x01, 0x2B),
            flag_dmt(b3, 0x80, 0x2C),
            flag_dmt(b3, 0x40, 0x39),
            flag_dmt(b3, 0x20, 0x3A),
            flag_dmt(b3, 0x10, 0x3B),
            flag_dmt(b3, 0x08, 0x3C),
            flag_dmt(b3, 0x04, 0x33),
            flag_dmt(b3, 0x02, 0x34),
            flag_dmt(b3, 0x01, 0x35),
            flag_dmt(b4, 0x80, 0x36),
            flag_dmt(b4, 0x40, 0x37),
            flag_dmt(b4, 0x20, 0x3E),
            flag_dmt(b4, 0x10, 0x3F),
            flag_dmt(b4, 0x08, 0x41),
            flag_dmt(b4, 0x04, 0x42),
            flag_dmt(b4, 0x02, 0x44),
            flag_dmt(b4, 0x01, 0x45),
            flag_dmt(b5, 0x80, 0x46),
            flag_dmt(b5, 0x40, 0x47),
            flag_dmt(b5, 0x20, 0x49),
            flag_dmt(b5, 0x10, 0x4A),
        ]
        .into_iter()
        .flatten()
    }

    /// Validates the Established Timings III descriptor.
    ///
    /// **Warnings**: version is not `0x0A`, or reserved bits/bytes are non-zero.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let bad_reserved = (self.raw[6] & 0x0F) != 0 || self.raw[7..13].iter().any(|&b| b != 0);
        Validation::new()
            .warn_if(self.raw[0] != 0x0A, WarningKind::EstTimingsVersionReserved)
            .warn_if(bad_reserved, WarningKind::EstTimingsReservedBits)
    }
}
