//! Display Color Management (DCM) Descriptor.
//!
//! Describes color correction coefficients for the display panel.
//! Uses tag 0xF9 and version 0x03.

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
    pub(super) const fn parse(raw: &[u8; 13]) -> Self {
        Self {
            red_a3: u16::from_le_bytes([raw[1], raw[2]]),
            red_a2: u16::from_le_bytes([raw[3], raw[4]]),
            green_a3: u16::from_le_bytes([raw[5], raw[6]]),
            green_a2: u16::from_le_bytes([raw[7], raw[8]]),
            blue_a3: u16::from_le_bytes([raw[9], raw[10]]),
            blue_a2: u16::from_le_bytes([raw[11], raw[12]]),
        }
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
