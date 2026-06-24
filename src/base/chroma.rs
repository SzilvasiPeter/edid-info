//! Chromaticity coordinates (bytes 25–34).
//!
//! 10-bit 2° [CIE 1931 xy coordinates](https://en.wikipedia.org/wiki/CIE_1931_color_space) for red, green, blue, and white point.
//! In simple terms, it stores where the red, green, blue, and [white](https://en.wikipedia.org/wiki/White_point) colors land on a standard color map.
//!
//! # Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 25   | Green-x LSBs + Red-x LSBs |
//! | 26   | White-x LSBs + Blue-x LSBs |
//! | 27   | Red-x MSBs |
//! | 28   | Red-y MSBs |
//! | 29   | Green-x MSBs |
//! | 30   | Green-y MSBs |
//! | 31   | Blue-x MSBs |
//! | 32   | Blue-y MSBs |
//! | 33   | White-x MSBs |
//! | 34   | White-y MSBs |

use crate::{
    bit::{get_bits, pack_bits},
    common::{BLOCK_LEN, Validation, WarningKind},
};
use core::fmt::{self, Display, Formatter};

/// Chromaticity coordinates offset in the base block.
pub const CHROMA_OFF: usize = 25;

/// Chromaticity coordinates length in bytes.
pub const CHROMA_LEN: usize = 10;

/// Chromaticity coordinates for red, green, blue, and white.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Chroma {
    red: Coord,
    green: Coord,
    blue: Coord,
    white: Coord,
}

impl Chroma {
    /// Initialize chromaticity coordinates from the base block.
    ///
    /// Each coordinate is a 10-bit value packed as:
    /// - 2 LSBs stored in bytes 25–26 (two bits per component)
    /// - 8 MSBs stored in bytes 27–34 (one byte per component)
    ///
    /// The LSB layout is:
    /// - Byte 25: red-x, red-y, green-x, green-y (from MSB to LSB pairs)
    /// - Byte 26: blue-x, blue-y, white-x, white-y (from MSB to LSB pairs)
    #[must_use]
    pub fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        let chroma = &raw[CHROMA_OFF..CHROMA_OFF + CHROMA_LEN];
        let green = (get_bits(chroma[0], 0, 1), get_bits(chroma[0], 2, 3));
        let red = (get_bits(chroma[0], 4, 5), get_bits(chroma[0], 6, 7));
        let white = (get_bits(chroma[1], 0, 1), get_bits(chroma[1], 2, 3));
        let blue = (get_bits(chroma[1], 4, 5), get_bits(chroma[1], 6, 7));
        Self {
            red: Coord {
                x: pack_bits(chroma[2], red.1, 2),
                y: pack_bits(chroma[3], red.0, 2),
            },
            green: Coord {
                x: pack_bits(chroma[4], green.1, 2),
                y: pack_bits(chroma[5], green.0, 2),
            },
            blue: Coord {
                x: pack_bits(chroma[6], blue.1, 2),
                y: pack_bits(chroma[7], blue.0, 2),
            },
            white: Coord {
                x: pack_bits(chroma[8], white.1, 2),
                y: pack_bits(chroma[9], white.0, 2),
            },
        }
    }

    /// Returns the red chromaticity coordinate.
    #[must_use]
    pub const fn red(&self) -> Coord {
        self.red
    }

    /// Returns the green chromaticity coordinate.
    #[must_use]
    pub const fn green(&self) -> Coord {
        self.green
    }

    /// Returns the blue chromaticity coordinate.
    #[must_use]
    pub const fn blue(&self) -> Coord {
        self.blue
    }

    /// Returns the white chromaticity coordinate.
    #[must_use]
    pub const fn white(&self) -> Coord {
        self.white
    }

    /// Returns true if all coordinates match [sRGB primaries](https://en.wikipedia.org/wiki/SRGB#Primaries) exactly.
    ///
    /// Expected 10-bit values (value / 1024 -> rounded xy):
    /// - red:   (655, 338) -> (0.6396, 0.3301) ~ (0.6400, 0.3300)
    /// - green: (307, 614) -> (0.2998, 0.5996) ~ (0.3000, 0.6000)
    /// - blue:  (154, 61)  -> (0.1504, 0.0596) ~ (0.1500, 0.0600)
    /// - white: (320, 337) -> (0.3125, 0.3291) ~ (0.3127, 0.3290)
    #[must_use]
    pub const fn is_srgb(&self) -> bool {
        (self.red.x == 655 && self.red.y == 338)
            && (self.green.x == 307 && self.green.y == 614)
            && (self.blue.x == 154 && self.blue.y == 61)
            && (self.white.x == 320 && self.white.y == 337)
    }

    /// Validates the chroma fields.
    #[must_use]
    pub const fn validate(&self, mono: bool) -> Validation {
        let rgb_non_zero = !((self.red.x == 0 && self.red.y == 0)
            && (self.green.x == 0 && self.green.y == 0)
            && (self.blue.x == 0 && self.blue.y == 0));
        Validation::new().warn_if(mono && rgb_non_zero, WarningKind::MonochromeHasNonZeroRgb)
    }
}

impl Display for Chroma {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "red={}, green={}, blue={}, white={}",
            self.red, self.green, self.blue, self.white
        )
    }
}

/// The color coordinates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

impl Coord {
    /// Returns the x coordinate value.
    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    /// Returns the y coordinate value.
    #[must_use]
    pub const fn y(&self) -> u16 {
        self.y
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
