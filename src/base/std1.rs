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
//!
//! If both bytes are 0x01, the entry is unused.
use crate::common::{AspectRatio, BLOCK_LEN, Validation};

pub const STANDARD_OFF: usize = 38;
pub const STANDARD_LEN: usize = 16;
pub const STANDARD_NUM: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Std1 {
    bytes: [u8; STANDARD_LEN],
    legacy: bool,
}

impl Std1 {
    /// Parses the standard timings from base block bytes.
    ///
    /// # Byte Structure
    /// - Byte 0: Horizontal pixels = (Value + 31) * 8
    /// - Byte 1:
    ///   - Bits 7-6: Aspect Ratio (00=16:10 or 1:1 legacy, 01=4:3, 10=5:4, 11=16:9)
    ///   - Bits 5-0: Vertical Refresh Rate = Value + 60 Hz
    #[must_use]
    pub fn new(raw: &[u8; BLOCK_LEN], legacy: bool) -> Self {
        let mut bytes = [0u8; STANDARD_LEN];
        bytes.copy_from_slice(&raw[STANDARD_OFF..STANDARD_OFF + STANDARD_LEN]);
        Self { bytes, legacy }
    }

    pub fn modes(&self) -> impl Iterator<Item = StdTiming> {
        self.bytes
            .chunks_exact(2)
            .filter_map(move |c| parse_std(c[0], c[1], self.legacy))
    }

    /// Validates the standard timings.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdTiming {
    pub width: u16,
    pub aspect: AspectRatio,
    pub vfreq: u8,
}

/// Parses a 2-byte standard timing entry.
#[must_use]
pub(super) const fn parse_std(x_byte: u8, y_byte: u8, legacy: bool) -> Option<StdTiming> {
    if x_byte == 0x01 && y_byte == 0x01 {
        return None;
    }
    let width = (x_byte as u16 + 31) * 8;
    let aspect = match y_byte & 0b1100_0000 {
        0b0000_0000 => {
            if legacy {
                AspectRatio::new(1, 1)
            } else {
                AspectRatio::new(16, 10)
            }
        }
        0b0100_0000 => AspectRatio::new(4, 3),
        0b1000_0000 => AspectRatio::new(5, 4),
        _ => AspectRatio::new(16, 9),
    };
    let vfreq = (y_byte & 0b0011_1111) + 60;
    Some(StdTiming {
        width,
        aspect,
        vfreq,
    })
}
