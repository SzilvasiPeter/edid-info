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
pub struct StdTimings {
    bytes: [u8; STANDARD_LEN],
    legacy: bool,
}

impl StdTimings {
    /// Initialize the standard timings from base block bytes.
    /// If the EDID version is less than 1.3, use legacy mode.
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

    /// Validates all standard timings in the base block.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let mut validations = Validation::new();
        for chunk in self.bytes.chunks_exact(2) {
            let empty = chunk[0] == 0x00 && chunk[1] == 0x00;
            validations = validations.fail_if(empty, FailureKind::InvalidEmptyStdTiming);
            if !empty && let Some(mode) = parse_std(chunk[0], chunk[1], self.legacy) {
                validations = validations.then(mode.validate());
            }
        }
        validations
    }
}

/// A single standard timing entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StandardTiming {
    bytes: [u8; 2],
    legacy: bool,
}

impl StandardTiming {
    /// Returns the standard timing code.
    #[must_use]
    pub const fn standard_timing_code(&self) -> u16 {
        u16::from_be_bytes(self.bytes)
    }

    /// Returns the horizontal addressable pixels (256–2288).
    #[must_use]
    pub const fn horizontal_active(&self) -> u16 {
        (self.bytes[0] as u16 + 31) * 8
    }

    /// Returns the image aspect ratio.
    #[must_use]
    pub const fn aspect_ratio(&self) -> AspectRatio {
        match (self.bytes[1] >> 6) & 0b11 {
            0b00 => {
                if self.legacy {
                    AspectRatio::new(1, 1)
                } else {
                    AspectRatio::new(16, 10)
                }
            }
            0b01 => AspectRatio::new(4, 3),
            0b10 => AspectRatio::new(5, 4),
            _ => AspectRatio::new(16, 9),
        }
    }

    /// Returns the vertical addressable lines.
    #[must_use]
    pub const fn vertical_active(&self) -> u16 {
        self.horizontal_active() * self.aspect_ratio().height() / self.aspect_ratio().width()
    }

    /// Returns the vertical refresh rate in Hz (60–123).
    #[must_use]
    pub const fn refresh_rate(&self) -> u8 {
        (self.bytes[1] & 0b0011_1111) + 60
    }

    /// Validates the standard timing entry.
    /// Note: Standard timings are limited to 256–2288 pixels and 123 Hz. Greater values must use DTDs or CVT descriptors.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new()
            .fail_if(
                self.horizontal_active() < 256 || self.horizontal_active() > 2288,
                FailureKind::StdTimingHorizontalLimit,
            )
            .fail_if(
                self.refresh_rate() > 123,
                FailureKind::StdTimingRefreshLimit,
            )
            .warn_if(
                !self.vertical_active().is_multiple_of(2),
                WarningKind::StdTimingOddVertical,
            )
    }
}

/// Parses a 2-byte standard timing entry. If both bytes are 0x01, the entry is unused.
///
/// # Byte Structure
/// - Byte 0: Horizontal pixels = (Value + 31) * 8
/// - Byte 1:
///   - Bits 7-6: Aspect Ratio (00=16:10 or 1:1 legacy, 01=4:3, 10=5:4, 11=16:9)
///   - Bits 5-0: Vertical Refresh Rate = Value + 60 Hz
#[must_use]
pub(super) const fn parse_std(byte1: u8, byte2: u8, legacy: bool) -> Option<StandardTiming> {
    if byte1 == 0x01 && byte2 == 0x01 {
        return None;
    }

    Some(StandardTiming {
        bytes: [byte1, byte2],
        legacy,
    })
}
