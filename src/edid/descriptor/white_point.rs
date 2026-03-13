//! White Point Data Descriptor.
//!
//! Contains up to two white point index entries with chromaticity
//! coordinates and gamma values. Uses tag 0xFB.
//!
//! # Structure
//!
//! Each white point entry (5 bytes):
//! - Byte 0: Index (0 = unused)
//! - Byte 1–2: White point x coordinate (10-bit)
//! - Byte 3: White point y coordinate (10-bit)
//! - Byte 4: Gamma (value - 1.0, scaled by 100)

use crate::edid::descriptor::DESC_LEN;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    index: u8,
    x_raw: u16,
    y_raw: u16,
    gamma_raw: u8,
}

impl Point {
    fn parse(raw: [u8; 5]) -> Option<Self> {
        if raw[0] == 0 {
            return None;
        }
        let lsb = raw[1];
        let x_raw = (u16::from(raw[2]) << 2) | u16::from((lsb >> 2) & 0b11);
        let y_raw = (u16::from(raw[3]) << 2) | u16::from(lsb & 0b11);
        Some(Self {
            index: raw[0],
            x_raw,
            y_raw,
            gamma_raw: raw[4],
        })
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn x_raw(&self) -> u16 {
        self.x_raw
    }

    #[must_use]
    pub const fn y_raw(&self) -> u16 {
        self.y_raw
    }

    #[must_use]
    pub const fn gamma_raw(&self) -> u8 {
        self.gamma_raw
    }

    #[must_use]
    pub fn gamma(&self) -> f32 {
        f32::from(self.gamma_raw) / 100.0 + 1.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WhitePoint {
    first: Option<Point>,
    second: Option<Point>,
    pad: [u8; 3],
}

impl WhitePoint {
    #[must_use]
    pub fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        if raw[0] != 0 || raw[1] != 0 || raw[2] != 0 || raw[3] != 0xFB || raw[4] != 0 {
            return None;
        }
        let first = Point::parse([raw[5], raw[6], raw[7], raw[8], raw[9]]);
        let second = Point::parse([raw[10], raw[11], raw[12], raw[13], raw[14]]);
        let pad = [raw[15], raw[16], raw[17]];
        Some(Self { first, second, pad })
    }

    #[must_use]
    pub const fn first(&self) -> Option<Point> {
        self.first
    }

    #[must_use]
    pub const fn second(&self) -> Option<Point> {
        self.second
    }

    #[must_use]
    pub const fn pad(&self) -> [u8; 3] {
        self.pad
    }
}
