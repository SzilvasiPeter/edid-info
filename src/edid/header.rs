//! EDID header (bytes 0–19).
//!
//! Contains the EDID magic pattern, manufacturer identification,
//! product code, serial number, and version/manufacture date.
//!
//! # Structure
//!
//! | Offset | Size | Description |
//! |--------|------|-------------|
//! | 0–7    | 8    | Header magic (0x00FFFFFFFFFFFF00) |
//! | 8–9    | 2    | Manufacturer ID (3-letter code, big-endian) |
//! | 10–11  | 2    | Product code |
//! | 12–15  | 4    | Serial number (little-endian) |
//! | 16     | 1    | Week of manufacture |
//! | 17     | 1    | Year of manufacture (offset from 1990) |
//! | 18     | 1    | EDID version major |
//! | 19     | 1    | EDID version minor |

use crate::edid::BLOCK_LEN;

/// Header structure containing manufacturer ID, product code, serial, and version info.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    magic: [u8; 8],
    maker: [char; 3],
    product: u16,
    serial: u32,
    week: u8,
    year: u16,
    major: u8,
    minor: u8,
}

impl Header {
    #[must_use]
    pub fn parse(raw: &[u8; BLOCK_LEN]) -> Self {
        let m = u16::from_be_bytes([raw[8], raw[9]]);
        Self {
            magic: raw[..8].try_into().unwrap_or([0; 8]),
            maker: [
                maker_char((m >> 10) & 0x1F),
                maker_char((m >> 5) & 0x1F),
                maker_char(m & 0x1F),
            ],
            product: u16::from_le_bytes([raw[10], raw[11]]),
            serial: u32::from_le_bytes([raw[12], raw[13], raw[14], raw[15]]),
            week: raw[16],
            year: 1990 + u16::from(raw[17]),
            major: raw[18],
            minor: raw[19],
        }
    }

    #[must_use]
    pub const fn magic(&self) -> [u8; 8] {
        self.magic
    }

    #[must_use]
    pub const fn maker(&self) -> [char; 3] {
        self.maker
    }

    #[must_use]
    pub const fn product(&self) -> u16 {
        self.product
    }

    #[must_use]
    pub const fn serial(&self) -> u32 {
        self.serial
    }

    #[must_use]
    pub const fn week(&self) -> u8 {
        self.week
    }

    #[must_use]
    pub const fn year(&self) -> u16 {
        self.year
    }

    #[must_use]
    pub const fn major(&self) -> u8 {
        self.major
    }

    #[must_use]
    pub const fn minor(&self) -> u8 {
        self.minor
    }
}

/// Converts a 5-bit EDID manufacturer code to ASCII.
/// EDID encodes letters as 1='A' through 26='Z', so we add 64 to get ASCII values.
fn maker_char(raw: u16) -> char {
    if (1..=26).contains(&raw) {
        char::from_u32(u32::from(raw) + 64).unwrap_or('?')
    } else {
        '?'
    }
}
