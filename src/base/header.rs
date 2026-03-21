//! EDID header (bytes 0–19).
//!
//! Contains the EDID header pattern, manufacturer ID, product code, serial number, manufacture date and version.
//!
//! # Structure
//!
//! | Offset | Size | Description |
//! |--------|------|-------------|
//! | 0–7    | 8    | Header pattern |
//! | 8–9    | 2    | Manufacturer ID |
//! | 10–11  | 2    | Product code |
//! | 12–15  | 4    | Serial number |
//! | 16     | 1    | Week of manufacture |
//! | 17     | 1    | Year of manufacture |
//! | 18     | 1    | EDID version major |
//! | 19     | 1    | EDID version minor |

use crate::common::{ErrorKind, Validation, WarningKind};

pub const HEADER_OFF: usize = 0;
pub const HEADER_LEN: usize = 20;

/// Header structure containing manufacturer ID, product code, serial, date and version info.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    pattern: [u8; 8],
    manufacturer: u16,
    product: u16,
    serial: u32,
    week: u8,
    year: u16,
    major: u8,
    minor: u8,
}

impl Header {
    /// Parses a header from 20 raw bytes.
    ///
    /// Byte sizes and endianness:
    /// - `pattern`: 8 bytes, raw
    /// - `manufacturer_ids`: 2 bytes, big-endian
    /// - `product`: 2 bytes, little-endian
    /// - `serial`: 4 bytes, little-endian
    /// - `week`: 1 byte, raw
    /// - `year`: 1 byte, raw (offset from 1990)
    /// - `major`: 1 byte, raw
    /// - `minor`: 1 byte, raw
    #[must_use]
    pub const fn new(raw: &[u8; HEADER_LEN]) -> Self {
        Self {
            pattern: [
                raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
            ],
            manufacturer: u16::from_be_bytes([raw[8], raw[9]]),
            product: u16::from_le_bytes([raw[10], raw[11]]),
            serial: u32::from_le_bytes([raw[12], raw[13], raw[14], raw[15]]),
            week: raw[16],
            year: 1990 + raw[17] as u16,
            major: raw[18],
            minor: raw[19],
        }
    }

    /// Fixed header pattern: `00 FF FF FF FF FF FF 00`
    #[must_use]
    pub const fn pattern(&self) -> [u8; 8] {
        self.pattern
    }

    /// Manufacturer's 3-letter code assigned by [UEFI forum](https://uefi.org/PNP_ID_List),
    /// which is a big-endian 16-bit value made up of three 5-bit letters.
    /// EDID encodes letters as 1='A' through 26='Z', so we add 64 to get ASCII values.
    ///
    /// | Bits (Bytes 8-9) | Description |
    /// |------------------|-------------|
    /// | 15 | Reserved |
    /// | 14–10 | First letter of manufacturer ID |
    /// | 9–5 | Second letter of manufacturer ID |
    /// | 4–0 | Third letter of manufacturer ID |
    #[must_use]
    pub const fn manufacturer(&self) -> [char; 3] {
        const fn to_char(bits: u8) -> char {
            (bits + 64) as char
        }
        let m1 = to_char(((self.manufacturer >> 10) & 0b11111) as u8);
        let m2 = to_char(((self.manufacturer >> 5) & 0b11111) as u8);
        let m3 = to_char((self.manufacturer & 0b11111) as u8);
        [m1, m2, m3]
    }

    /// Manufacturer product code. 16-bit hex number, little-endian.
    #[must_use]
    pub const fn product(&self) -> u16 {
        self.product
    }

    /// Serial number. 32 bits, little-endian.
    #[must_use]
    pub const fn serial(&self) -> u32 {
        self.serial
    }

    /// Week of manufacture; or `None` if model year flag (0xFF) is set.
    /// A value of 0 means the week is unspecified.
    /// [Week numbering](https://en.wikipedia.org/wiki/Week#Numbering) is not consistent between manufacturers.
    #[must_use]
    pub const fn week(&self) -> Option<u8> {
        if self.week == 0xFF {
            None
        } else {
            Some(self.week)
        }
    }

    /// Year of manufacture, or year of model, if model year flag is set. Year = datavalue + 1990.
    #[must_use]
    pub const fn year(&self) -> u16 {
        self.year
    }

    /// EDID version, usually `01` (for 1.3 and 1.4)
    #[must_use]
    pub const fn major(&self) -> u8 {
        self.major
    }

    /// EDID revision, usually `03` (for 1.3) or `04` (for 1.4)
    #[must_use]
    pub const fn minor(&self) -> u8 {
        self.minor
    }

    /// Validates the header.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        const fn is_letter(code: u16) -> bool {
            code >= 1 && code <= 26
        }
        let m1 = (self.manufacturer >> 10) & 0b11111;
        let m2 = (self.manufacturer >> 5) & 0b11111;
        let m3 = self.manufacturer & 0b11111;
        Validation::new()
            .err_if(
                !is_letter(m1) || !is_letter(m2) || !is_letter(m3),
                ErrorKind::HeaderMfrInvalidBits,
            )
            .err_if(
                self.week > 54 && self.week != 0xFF,
                ErrorKind::HeaderWeekInvalid,
            )
            .err_if(self.major == 0, ErrorKind::HeaderMajorInvalid)
            .warn_if(
                self.manufacturer & 0b1000_0000_0000_0000 != 0,
                WarningKind::HeaderMfrReservedSet,
            )
            .warn_if(self.product == 0, WarningKind::HeaderProductInvalid)
            .warn_if(self.serial == 0, WarningKind::HeaderSerialInvalid)
            .warn_if(
                self.major != 1 || self.minor != 4,
                WarningKind::HeaderVersionDeprecated,
            )
    }
}
