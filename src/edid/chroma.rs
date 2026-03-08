use crate::edid::BLOCK_LEN;
use crate::edid::bits::{u2_from_masks, u10_hi};

pub const CHROMA_OFF: usize = 25;
pub const CHROMA_LEN: usize = 10;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Chroma {
    red: Coord,
    green: Coord,
    blue: Coord,
    white: Coord,
}

impl Chroma {
    #[must_use]
    pub fn parse_base(raw: &[u8; BLOCK_LEN]) -> Self {
        let mut out = [0; CHROMA_LEN];
        out.copy_from_slice(&raw[CHROMA_OFF..CHROMA_OFF + CHROMA_LEN]);
        Self::parse(&out)
    }

    #[must_use]
    pub const fn parse(raw: &[u8; CHROMA_LEN]) -> Self {
        let rg = raw[0];
        let bw = raw[1];
        Self {
            red: Coord {
                x: u10_hi(raw[2], u2_from_masks(rg, 0b1000_0000, 0b0100_0000)),
                y: u10_hi(raw[3], u2_from_masks(rg, 0b0010_0000, 0b0001_0000)),
            },
            green: Coord {
                x: u10_hi(raw[4], u2_from_masks(rg, 0b0000_1000, 0b0000_0100)),
                y: u10_hi(raw[5], u2_from_masks(rg, 0b0000_0010, 0b0000_0001)),
            },
            blue: Coord {
                x: u10_hi(raw[6], u2_from_masks(bw, 0b1000_0000, 0b0100_0000)),
                y: u10_hi(raw[7], u2_from_masks(bw, 0b0010_0000, 0b0001_0000)),
            },
            white: Coord {
                x: u10_hi(raw[8], u2_from_masks(bw, 0b0000_1000, 0b0000_0100)),
                y: u10_hi(raw[9], u2_from_masks(bw, 0b0000_0010, 0b0000_0001)),
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
}
