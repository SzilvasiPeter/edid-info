//! Header information (bytes 0–19).
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

use crate::common::{BLOCK_LEN, FailureKind, Validation, Version, WarningKind};

/// Header offset in the base block.
pub const HEADER_OFF: usize = 0;

/// Header length in bytes.
pub const HEADER_LEN: usize = 20;

const YEAR_OFFSET: u16 = 1990;
const WEEK_MODEL_YEAR_FLAG: u8 = 0xFF;

/// Date of manufacture or model year.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DateInfo {
    /// Manufacture week (1–54) and year.
    Manufacture { week: u8, year: u16 },
    /// Model year (used when week is 0xFF).
    ModelYear { year: u16 },
}

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
    /// Parses the header from base block bytes.
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
    pub fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        let header = &raw[HEADER_OFF..HEADER_OFF + HEADER_LEN];
        Self {
            pattern: [
                header[0], header[1], header[2], header[3], header[4], header[5], header[6],
                header[7],
            ],
            manufacturer: u16::from_be_bytes([header[8], header[9]]),
            product: u16::from_le_bytes([header[10], header[11]]),
            serial: u32::from_le_bytes([header[12], header[13], header[14], header[15]]),
            week: header[16],
            year: YEAR_OFFSET + u16::from(header[17]),
            major: header[18],
            minor: header[19],
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
    #[must_use]
    pub const fn manufacturer(&self) -> [char; 3] {
        let (m1, m2, m3) = decode(self.manufacturer);
        [(m1 + 64) as char, (m2 + 64) as char, (m3 + 64) as char]
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

    /// Manufacture or model date information.
    ///
    /// If week is `0xFF`, it represents a model year. Week 0 is unspecified.
    /// Year is calculated as `value + 1990`.
    #[must_use]
    pub const fn date(&self) -> DateInfo {
        if self.week == WEEK_MODEL_YEAR_FLAG {
            DateInfo::ModelYear { year: self.year }
        } else {
            DateInfo::Manufacture {
                week: self.week,
                year: self.year,
            }
        }
    }

    /// EDID version information.
    #[must_use]
    pub const fn version(&self) -> Version {
        Version {
            major: self.major,
            minor: self.minor,
        }
    }

    /// Validates the header blocks.
    ///
    /// Checks for:
    /// - **Failures**: Invalid manufacturer ID characters, invalid week number, or zero major version.
    /// - **Warnings**: Reserved manufacturer bits set, zero product/serial codes, or non-1.4 EDID versions.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        const fn is_letter(code: u8) -> bool {
            code >= 1 && code <= 26
        }

        let (m1, m2, m3) = decode(self.manufacturer);
        Validation::new()
            .fail_if(
                !is_letter(m1) || !is_letter(m2) || !is_letter(m3),
                FailureKind::HeaderMfrInvalidBits,
            )
            .fail_if(
                self.week > 54 && self.week != WEEK_MODEL_YEAR_FLAG,
                FailureKind::HeaderWeekInvalid,
            )
            .fail_if(self.major == 0, FailureKind::HeaderMajorInvalid)
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

impl core::fmt::Display for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let [m1, m2, m3] = self.manufacturer();
        write!(
            f,
            "Manufacturer: {m1}{m2}{m3}, Product: {:04X}, Version: {}",
            self.product,
            self.version()
        )
    }
}

/// Decodes the ID into three 5-bit values.
///
/// | Bits (Bytes 8-9) | Description |
/// |------------------|-------------|
/// | 15 | Reserved |
/// | 14–10 | First letter of manufacturer ID |
/// | 9–5 | Second letter of manufacturer ID |
/// | 4–0 | Third letter of manufacturer ID |
#[must_use]
const fn decode(manufacturer: u16) -> (u8, u8, u8) {
    (
        ((manufacturer >> 10) & 0b11111) as u8,
        ((manufacturer >> 5) & 0b11111) as u8,
        (manufacturer & 0b11111) as u8,
    )
}
