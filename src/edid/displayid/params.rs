//! `DisplayID` Display Parameters Data Block.
//!
//! Contains core display characteristics including physical dimensions,
//! native resolution, color primaries, luminance range, and gamma.
//!
//! # Key Fields
//!
//! - Physical size (mm)
//! - Active pixels (horizontal/vertical)
//! - Color chromaticity coordinates (12-bit)
//! - Luminance values (max/min in cd/m²)
//! - Color depth and technology type
//! - Gamma curve

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayParams {
    h_size: u16,
    v_size: u16,
    h_pixels: u16,
    v_pixels: u16,
    features: u8,
    red_x: u16,
    red_y: u16,
    green_x: u16,
    green_y: u16,
    blue_x: u16,
    blue_y: u16,
    white_x: u16,
    white_y: u16,
    max_lum_full: u16,
    max_lum_10: u16,
    min_lum: u16,
    depth_tech: u8,
    gamma: u8,
}

impl DisplayParams {
    #[must_use]
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 29 {
            return None;
        }

        Some(Self {
            h_size: u16::from_le_bytes([data[0], data[1]]),
            v_size: u16::from_le_bytes([data[2], data[3]]),
            h_pixels: u16::from_le_bytes([data[4], data[5]]),
            v_pixels: u16::from_le_bytes([data[6], data[7]]),
            features: data[8],
            red_x: parse_chroma_12(data[9], data[10]),
            red_y: parse_chroma_12_hi(data[10], data[11]),
            green_x: parse_chroma_12(data[12], data[13]),
            green_y: parse_chroma_12_hi(data[13], data[14]),
            blue_x: parse_chroma_12(data[15], data[16]),
            blue_y: parse_chroma_12_hi(data[16], data[17]),
            white_x: parse_chroma_12(data[18], data[19]),
            white_y: parse_chroma_12_hi(data[19], data[20]),
            max_lum_full: u16::from_le_bytes([data[21], data[22]]),
            max_lum_10: u16::from_le_bytes([data[23], data[24]]),
            min_lum: u16::from_le_bytes([data[25], data[26]]),
            depth_tech: data[27],
            gamma: data[28],
        })
    }

    #[must_use]
    pub const fn h_size_raw(&self) -> u16 {
        self.h_size
    }
    #[must_use]
    pub const fn v_size_raw(&self) -> u16 {
        self.v_size
    }
    #[must_use]
    pub const fn h_pixels(&self) -> u16 {
        self.h_pixels
    }
    #[must_use]
    pub const fn v_pixels(&self) -> u16 {
        self.v_pixels
    }

    #[must_use]
    pub const fn color_depth_bpc(&self) -> u8 {
        match self.depth_tech & 0x07 {
            1 => 6,
            2 => 8,
            3 => 10,
            4 => 12,
            5 => 16,
            _ => 0,
        }
    }

    #[must_use]
    pub fn gamma(&self) -> Option<f32> {
        if self.gamma == 255 {
            None
        } else {
            Some(f32::from(self.gamma) / 100.0 + 1.0)
        }
    }
}

const fn parse_chroma_12(lsb: u8, mid: u8) -> u16 {
    (((mid & 0x0F) as u16) << 8) | lsb as u16
}

const fn parse_chroma_12_hi(mid: u8, msb: u8) -> u16 {
    ((msb as u16) << 4) | (mid >> 4) as u16
}
