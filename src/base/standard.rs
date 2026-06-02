//! Standard Timing information (bytes 38–53).
//!
//! Up to 8 2-byte fields describing standard display modes.
//! Unused fields are filled with 01 01 hex.
//!
//! # Structure
//!
//! Each 2-byte entry encodes:
//! - Byte N: Horizontal resolution code ((width/8) - 31)
//! - Byte N+1: Aspect ratio (bits 7–6) + vertical refresh (bits 5–0)
//!
//! | Offset | Count | Description |
//! |--------|-------|-------------|
//! | 38–53  | 8×2   | Standard timing descriptors |
use crate::common::{AspectRatio, BLOCK_LEN, FailureKind, Validation, WarningKind};

/// Standard timings offset in the base block.
pub const STANDARD_OFF: usize = 38;

/// Standard timings length in bytes.
pub const STANDARD_LEN: usize = 16;

/// Standard Timing information for the base block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StandardTimings {
    bytes: [u8; STANDARD_LEN],
    legacy: bool,
}

impl StandardTimings {
    /// Initialize the standard timings from base block bytes.
    #[must_use]
    pub fn new(raw: &[u8; BLOCK_LEN], legacy: bool) -> Self {
        let mut bytes = [0u8; STANDARD_LEN];
        bytes.copy_from_slice(&raw[STANDARD_OFF..STANDARD_OFF + STANDARD_LEN]);
        Self { bytes, legacy }
    }

    /// Returns an iterator over the defined standard timings.
    pub fn iter(&self) -> impl Iterator<Item = StandardTiming> {
        self.bytes
            .chunks_exact(2)
            .filter_map(move |c| parse_std(c[0], c[1], self.legacy))
    }

    /// Validates all standard timings in the block.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let mut v = Validation::new();
        for c in self.bytes.chunks_exact(2) {
            let empty = c[0] == 0x00 && c[1] == 0x00;
            v = v.fail_if(empty, FailureKind::StdTimingEmptyInvalid);
            if !empty && let Some(mode) = parse_std(c[0], c[1], self.legacy) {
                v = v.then(mode.validate());
            }
        }
        v
    }
}

/// A single standard timing entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StandardTiming {
    /// Horizontal addressable pixels (256–2288).
    pub horizontal_active: u16,
    /// Vertical addressable lines (calculated from aspect ratio).
    pub vertical_active: u16,
    /// Image aspect ratio.
    pub aspect_ratio: AspectRatio,
    /// Vertical refresh rate in Hz (60–123).
    pub refresh_rate: u8,
}

impl StandardTiming {
    /// Validates the standard timing entry.
    /// Note: Standard timings are limited to 256–2288 pixels and 123 Hz. Greater values must use DTDs or CVT descriptors.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new()
            .fail_if(
                self.horizontal_active < 256 || self.horizontal_active > 2288,
                FailureKind::StdTimingHorizontalLimit,
            )
            .fail_if(self.refresh_rate > 123, FailureKind::StdTimingRefreshLimit)
            .warn_if(
                !self.vertical_active.is_multiple_of(2),
                WarningKind::StdTimingOddVertical,
            )
    }
}

/// Parses a 2-byte standard timing entry. If both bytes are 0x01, the entry is unused.
///
///  # Byte Structure
/// - Byte 0: Horizontal pixels = (Value + 31) * 8
/// - Byte 1:
///   - Bits 7-6: Aspect Ratio (00=16:10 or 1:1 legacy, 01=4:3, 10=5:4, 11=16:9)
///   - Bits 5-0: Vertical Refresh Rate = Value + 60 Hz
#[must_use]
pub(super) fn parse_std(byte1: u8, byte2: u8, legacy: bool) -> Option<StandardTiming> {
    if byte1 == 0x01 && byte2 == 0x01 {
        return None;
    }

    let horizontal_active = (u16::from(byte1) + 31) * 8;
    let (aspect_ratio, vertical_active) = match (byte2 >> 6) & 0b11 {
        0b00 => {
            if legacy {
                (AspectRatio::new(1, 1), horizontal_active)
            } else {
                (AspectRatio::new(16, 10), horizontal_active * 10 / 16)
            }
        }
        0b01 => (AspectRatio::new(4, 3), horizontal_active * 3 / 4),
        0b10 => (AspectRatio::new(5, 4), horizontal_active * 4 / 5),
        _ => (AspectRatio::new(16, 9), horizontal_active * 9 / 16),
    };
    let refresh_rate = (byte2 & 0b0011_1111) + 60;

    Some(StandardTiming {
        horizontal_active,
        vertical_active,
        aspect_ratio,
        refresh_rate,
    })
}
