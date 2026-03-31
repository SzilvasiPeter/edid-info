//! Chromaticity coordinates (bytes 25–34).
//!
//! 10-bit 2° [CIE 1931 xy coordinates](https://en.wikipedia.org/wiki/CIE_1931_color_space) for red, green, blue, and white point.
//!
//! # Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 25   | Red-x LSBs + Green-x LSBs |
//! | 26   | Blue-x LSBs + White-x LSBs |
//! | 27   | Red-x MSBs |
//! | 28   | Red-y MSBs |
//! | 29   | Green-x MSBs |
//! | 30   | Green-y MSBs |
//! | 31   | Blue-x MSBs |
//! | 32   | Blue-y MSBs |
//! | 33   | White-x MSBs |
//! | 34   | White-y MSBs |

use crate::{
    bit::{u2_from_masks, u10_hi},
    common::{BLOCK_LEN, Validation},
};

pub const CHROMA_OFF: usize = 25;
pub const CHROMA_LEN: usize = 10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Chroma {
    red: Coord,
    green: Coord,
    blue: Coord,
    white: Coord,
}

impl Chroma {
    // TODO: Add byte size and byte packing info in docstring.
    #[must_use]
    pub fn parse(raw: &[u8; BLOCK_LEN]) -> Self {
        let chroma = &raw[CHROMA_OFF..CHROMA_OFF + CHROMA_LEN];
        // TODO: Make the bit packing more readable.
        // For example, first gather the LSBs and MSBs, then combine them
        // Use packing LSBs + MSBs so the indexing is easier to follow.
        // Mask the LSBs before packing.
        // Use better name for the bit packing.
        let rg = chroma[0];
        let bw = chroma[1];
        Self {
            red: Coord {
                x: u10_hi(chroma[2], u2_from_masks(rg, 0b1000_0000, 0b0100_0000)),
                y: u10_hi(chroma[3], u2_from_masks(rg, 0b0010_0000, 0b0001_0000)),
            },
            green: Coord {
                x: u10_hi(chroma[4], u2_from_masks(rg, 0b0000_1000, 0b0000_0100)),
                y: u10_hi(chroma[5], u2_from_masks(rg, 0b0000_0010, 0b0000_0001)),
            },
            blue: Coord {
                x: u10_hi(chroma[6], u2_from_masks(bw, 0b1000_0000, 0b0100_0000)),
                y: u10_hi(chroma[7], u2_from_masks(bw, 0b0010_0000, 0b0001_0000)),
            },
            white: Coord {
                x: u10_hi(chroma[8], u2_from_masks(bw, 0b0000_1000, 0b0000_0100)),
                y: u10_hi(chroma[9], u2_from_masks(bw, 0b0000_0010, 0b0000_0001)),
            },
        }
    }

    #[must_use]
    pub const fn red(&self) -> Coord {
        self.red
    }

    #[must_use]
    pub const fn green(&self) -> Coord {
        self.green
    }

    #[must_use]
    pub const fn blue(&self) -> Coord {
        self.blue
    }

    #[must_use]
    pub const fn white(&self) -> Coord {
        self.white
    }

    /// Validates the chroma fields.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        todo!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coord {
    x: u16,
    y: u16,
}

impl Coord {
    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> u16 {
        self.y
    }
}
