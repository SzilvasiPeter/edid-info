//! Display Color Management (DCM) Descriptor.
//!
//! Describes color correction coefficients for the display panel.
//! Uses tag 0xF9 and version 0x03.

use crate::edid::DESC_LEN;

const VERSION: u8 = 0x03;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    red_a3: u16,
    red_a2: u16,
    green_a3: u16,
    green_a2: u16,
    blue_a3: u16,
    blue_a2: u16,
}

impl Color {
    #[must_use]
    pub const fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        if raw[0] != 0 || raw[1] != 0 || raw[2] != 0 || raw[3] != 0xF9 || raw[4] != 0 {
            return None;
        }
        if raw[5] != VERSION {
            return None;
        }
        Some(Self {
            red_a3: u16::from_le_bytes([raw[6], raw[7]]),
            red_a2: u16::from_le_bytes([raw[8], raw[9]]),
            green_a3: u16::from_le_bytes([raw[10], raw[11]]),
            green_a2: u16::from_le_bytes([raw[12], raw[13]]),
            blue_a3: u16::from_le_bytes([raw[14], raw[15]]),
            blue_a2: u16::from_le_bytes([raw[16], raw[17]]),
        })
    }

    #[must_use]
    pub const fn red_a3(&self) -> u16 {
        self.red_a3
    }

    #[must_use]
    pub const fn red_a2(&self) -> u16 {
        self.red_a2
    }

    #[must_use]
    pub const fn green_a3(&self) -> u16 {
        self.green_a3
    }

    #[must_use]
    pub const fn green_a2(&self) -> u16 {
        self.green_a2
    }

    #[must_use]
    pub const fn blue_a3(&self) -> u16 {
        self.blue_a3
    }

    #[must_use]
    pub const fn blue_a2(&self) -> u16 {
        self.blue_a2
    }
}
