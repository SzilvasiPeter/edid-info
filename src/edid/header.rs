//! EDID header (bytes 0–19).
//!
//! Contains the EDID header pattern, manufacturer ID, product code, serial number, manufacture date and version.
//!
//! # Structure
//!
//! | Offset | Size | Description |
//! |--------|------|-------------|
//! | 0–7    | 8    | Header pattern (0x00FFFFFFFFFFFF00) |
//! | 8–9    | 2    | Manufacturer ID (3-letter code, big-endian) |
//! | 10–11  | 2    | Product code |
//! | 12–15  | 4    | Serial number (little-endian) |
//! | 16     | 1    | Week of manufacture |
//! | 17     | 1    | Year of manufacture (offset from 1990) |
//! | 18     | 1    | EDID version major |
//! | 19     | 1    | EDID version minor |

pub const HEADER_OFF: usize = 0;
pub const HEADER_LEN: usize = 20;

/// Header structure containing manufacturer ID, product code, serial, date and version info.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    pattern: [u8; 8],
    manufacturer_ids: [u8; 2],
    product: [u8; 2],
    serial: [u8; 4],
    week: u8,
    year: u8,
    major: u8,
    minor: u8,
}

impl Header {
    #[must_use]
    pub const fn parse(raw: &[u8; HEADER_LEN]) -> Self {
        Self {
            pattern: [
                raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
            ],
            manufacturer_ids: [raw[8], raw[9]],
            product: [raw[10], raw[11]],
            serial: [raw[12], raw[13], raw[14], raw[15]],
            week: raw[16],
            year: raw[17],
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
    ///
    /// Invalid values (0 or 27–31) are converted to '?'.
    #[must_use]
    pub fn manufacturer(&self) -> [char; 3] {
        let ids = u16::from_be_bytes(self.manufacturer_ids);
        let to_char = |bits| {
            if !(1..=26).contains(&bits) {
                return '?';
            }
            char::from_u32(u32::from(bits) + 64).unwrap_or('?')
        };
        let first = to_char((ids >> 10) & 0b11111);
        let second = to_char((ids >> 5) & 0b11111);
        let third = to_char(ids & 0b11111);
        [first, second, third]
    }

    /// Manufacturer product code. 16-bit hex number, little-endian. For example, "C0CF".
    #[must_use]
    pub const fn product(&self) -> u16 {
        u16::from_le_bytes(self.product)
    }

    /// Serial number. 32 bits, little-endian.
    #[must_use]
    pub const fn serial(&self) -> u32 {
        u32::from_le_bytes(self.serial)
    }

    /// Week of manufacture; or `FF` model year flag.
    /// [Week numbering](https://en.wikipedia.org/wiki/Week#Numbering) is not consistent between manufacturers.
    #[must_use]
    pub const fn week(&self) -> u8 {
        self.week
    }

    /// Year of manufacture, or year of model, if model year flag is set. Year = datavalue + 1990.
    #[must_use]
    pub const fn year(&self) -> u16 {
        1990 + self.year as u16
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
}
