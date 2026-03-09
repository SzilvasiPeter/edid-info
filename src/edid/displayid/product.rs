//! `DisplayID` Product Identification Data Block.
//!
//! Contains vendor identification, product code, serial number,
//! manufacture date, and optional display name.
//!
//! # Structure
//!
//! | Bytes | Description |
//! |-------|-------------|
//! | 0–2   | IEEE OUI (vendor ID) |
//! | 3–4   | Product code |
//! | 5–8   | Serial number (optional) |
//! | 9     | Week of manufacture |
//! | 10    | Year of manufacture (offset from 2000) |
//! | 11    | Display name length |
//! | 12+   | Display name (ASCII) |

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProductIdent<'a> {
    oui: [u8; 3],
    product: u16,
    serial: Option<u32>,
    week: u8,
    year: u16,
    name: Option<&'a str>,
}

impl<'a> ProductIdent<'a> {
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < 12 {
            return None;
        }

        let oui = [data[0], data[1], data[2]];
        let product = u16::from_le_bytes([data[3], data[4]]);
        let serial = if data.len() >= 9 {
            Some(u32::from_le_bytes([data[5], data[6], data[7], data[8]]))
        } else {
            None
        };

        let week = data[9];
        let year_raw = data[10];
        let year = if year_raw != 0 {
            2000 + u16::from(year_raw)
        } else {
            0
        };

        let name_len = data[11] as usize;
        let name = if name_len > 0 && data.len() >= 12 + name_len {
            core::str::from_utf8(&data[12..12 + name_len]).ok()
        } else {
            None
        };

        Some(Self {
            oui,
            product,
            serial,
            week,
            year,
            name,
        })
    }

    #[must_use]
    pub const fn oui(&self) -> [u8; 3] {
        self.oui
    }

    #[must_use]
    pub const fn product(&self) -> u16 {
        self.product
    }

    #[must_use]
    pub const fn serial(&self) -> Option<u32> {
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
    pub const fn name(&self) -> Option<&'a str> {
        self.name
    }
}
