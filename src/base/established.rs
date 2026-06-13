//! Established timing I & II bitmap (bytes 35–37).
//!
//! This optional field is a compact bitmap of factory-supported legacy timings, where each bit set to `1` means that exact mode is supported.
//! Byte 37 bits 6-0 are manufacturer-defined flags that must not be used to infer display limits.
//!
//! # Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 35   | Established timings (common legacy modes) |
//! | 36   | Established timings (common legacy modes) |
//! | 37   | 1152x870 at 75 Hz + 7 manufacturer-defined flags |

use super::dmt::{Dmt, find_dmt};
use crate::common::{BLOCK_LEN, Polarity, SyncPolarity, Timing};

/// Established timings offset in the base block.
pub const ESTABLISHED_OFF: usize = 35;

/// Established timings length in bytes.
pub const ESTABLISHED_LEN: usize = 3;

/// Parsed established timings and manufacturer-defined flags.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstTimings {
    bytes: [u8; 3],
}

impl EstTimings {
    /// Initialize established timings from base block bytes.
    ///
    /// Bytes:
    /// - 35-36: established timing flags
    /// - 37 bit 7: 1152x870 at 75 Hz
    /// - 37 bits 6-0: manufacturer-defined flags
    #[must_use]
    pub const fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        Self {
            bytes: [
                raw[ESTABLISHED_OFF],
                raw[ESTABLISHED_OFF + 1],
                raw[ESTABLISHED_OFF + 2],
            ],
        }
    }

    /// Supported established timings.
    pub fn iter(&self) -> impl Iterator<Item = Dmt> {
        let bytes = self.bytes;
        [
            flag_custom(bytes[0], 0x80, IBM_720X400_70),
            flag_custom(bytes[0], 0x40, IBM_720X400_88),
            flag_dmt(bytes[0], 0x20, 0x04),
            flag_custom(bytes[0], 0x10, APPLE_640X480_67),
            flag_dmt(bytes[0], 0x08, 0x05),
            flag_dmt(bytes[0], 0x04, 0x06),
            flag_dmt(bytes[0], 0x02, 0x08),
            flag_dmt(bytes[0], 0x01, 0x09),
            flag_dmt(bytes[1], 0x80, 0x0A),
            flag_dmt(bytes[1], 0x40, 0x0B),
            flag_custom(bytes[1], 0x20, APPLE_832X624_75),
            flag_dmt(bytes[1], 0x10, 0x0F),
            flag_dmt(bytes[1], 0x08, 0x10),
            flag_dmt(bytes[1], 0x04, 0x11),
            flag_dmt(bytes[1], 0x02, 0x12),
            flag_dmt(bytes[1], 0x01, 0x24),
            flag_custom(bytes[2], 0x80, APPLE_1152X870_75),
        ]
        .into_iter()
        .flatten()
    }

    /// Raw manufacturer-defined flags from byte 37 bits 6-0.
    #[must_use]
    pub const fn manufacturer_bits(&self) -> u8 {
        self.bytes[2] & 0x7F
    }
}

/// Returns a DMT timing if the corresponding bit in the byte is set.
pub(crate) const fn flag_dmt(byte: u8, mask: u8, id: u8) -> Option<Dmt> {
    if (byte & mask) == 0 {
        return None;
    }
    find_dmt(id)
}

const fn flag_custom(byte: u8, mask: u8, val: Dmt) -> Option<Dmt> {
    if (byte & mask) != 0 { Some(val) } else { None }
}

const IBM_720X400_70: Dmt = Dmt {
    id: 0x00,
    std_code: None,
    cvt_code: None,
    pixel_clock_khz: 28_320,
    interlaced: false,
    horizontal: Timing::new(720, 180, 18, 108, 0),
    vertical: Timing::new(400, 49, 21, 2, 0),
    sync: SyncPolarity {
        horizontal: Polarity::Negative,
        vertical: Polarity::Positive,
    },
};

const IBM_720X400_88: Dmt = Dmt {
    id: 0x00,
    std_code: None,
    cvt_code: None,
    pixel_clock_khz: 35_500,
    interlaced: false,
    horizontal: Timing::new(720, 180, 18, 108, 0),
    vertical: Timing::new(400, 49, 12, 2, 0),
    sync: SyncPolarity {
        horizontal: Polarity::Negative,
        vertical: Polarity::Positive,
    },
};

const APPLE_640X480_67: Dmt = Dmt {
    id: 0x00,
    std_code: None,
    cvt_code: None,
    pixel_clock_khz: 30_240,
    interlaced: false,
    horizontal: Timing::new(640, 224, 64, 96, 0),
    vertical: Timing::new(480, 45, 3, 3, 0),
    sync: SyncPolarity {
        horizontal: Polarity::Negative,
        vertical: Polarity::Negative,
    },
};

const APPLE_832X624_75: Dmt = Dmt {
    id: 0x00,
    std_code: None,
    cvt_code: None,
    pixel_clock_khz: 57_284,
    interlaced: false,
    horizontal: Timing::new(832, 320, 32, 64, 0),
    vertical: Timing::new(624, 43, 1, 3, 0),
    sync: SyncPolarity {
        horizontal: Polarity::Negative,
        vertical: Polarity::Negative,
    },
};

const APPLE_1152X870_75: Dmt = Dmt {
    id: 0x00,
    std_code: None,
    cvt_code: None,
    pixel_clock_khz: 100_000,
    interlaced: false,
    horizontal: Timing::new(1152, 304, 48, 128, 0),
    vertical: Timing::new(870, 45, 3, 3, 0),
    sync: SyncPolarity {
        horizontal: Polarity::Positive,
        vertical: Polarity::Positive,
    },
};
