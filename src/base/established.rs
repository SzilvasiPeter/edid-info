//! Established timing I & II bitmap (bytes 35–37).
//!
//! Supported bitmap for (formerly) very common timing modes.
//!
//! # Structure
//! | Byte | Description |
//! |------|-------------|
//! | 35   | 720×400 - 800×600 displays |
//! | 36   | 800×600 - 1280×1024 displays |
//! | 37   | 1152×870 + 7 manufacturer-specific display modes |

use super::dmt::{Dmt, find_dmt};
use crate::common::{BLOCK_LEN, Polarity, SyncPolarity, Timing};

pub const ESTABLISHED_OFF: usize = 35;
pub const ESTABLISHED_LEN: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstablishedLegacy {
    timings: [Option<Dmt>; 17],
    manufacturer_bits: u8,
}

impl EstablishedLegacy {
    #[must_use]
    pub fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        let established = &raw[ESTABLISHED_OFF..ESTABLISHED_OFF + ESTABLISHED_LEN];

        Self {
            timings: [
                flag_custom(established[0], 0x80, IBM_720X400_70),
                flag_custom(established[0], 0x40, IBM_720X400_88),
                flag_dmt(established[0], 0x20, 0x04),
                flag_custom(established[0], 0x10, APPLE_640X480_67),
                flag_dmt(established[0], 0x08, 0x05),
                flag_dmt(established[0], 0x04, 0x06),
                flag_dmt(established[0], 0x02, 0x08),
                flag_dmt(established[0], 0x01, 0x09),
                flag_dmt(established[1], 0x80, 0x0A),
                flag_dmt(established[1], 0x40, 0x0B),
                flag_custom(established[1], 0x20, APPLE_832X624_75),
                flag_dmt(established[1], 0x10, 0x0F),
                flag_dmt(established[1], 0x08, 0x10),
                flag_dmt(established[1], 0x04, 0x11),
                flag_dmt(established[1], 0x02, 0x12),
                flag_dmt(established[1], 0x01, 0x24),
                flag_custom(established[2], 0x80, APPLE_1152X870_75),
            ],
            manufacturer_bits: established[2] & 0x7F,
        }
    }

    #[must_use]
    pub const fn supported(&self) -> [Option<Dmt>; 17] {
        self.timings
    }

    #[must_use]
    pub const fn manufacturer_bits(&self) -> u8 {
        self.manufacturer_bits
    }
}

const fn flag_custom(byte: u8, mask: u8, val: Dmt) -> Option<Dmt> {
    if (byte & mask) != 0 { Some(val) } else { None }
}

const fn flag_dmt(byte: u8, mask: u8, id: u8) -> Option<Dmt> {
    if (byte & mask) == 0 {
        return None;
    }
    find_dmt(id)
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
