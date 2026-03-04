use crate::edid::base::BASE_LEN;

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
    pub fn parse_base(raw: &[u8; BASE_LEN]) -> Self {
        let mut out = [0; CHROMA_LEN];
        out.copy_from_slice(&raw[CHROMA_OFF..CHROMA_OFF + CHROMA_LEN]);
        Self::parse(&out)
    }

    #[must_use]
    pub const fn parse(raw: &[u8; CHROMA_LEN]) -> Self {
        const fn bit(value: u8, mask: u8) -> u16 {
            if (value & mask) != 0 { 1 } else { 0 }
        }
        const fn pair(value: u8, high: u8, low: u8) -> u16 {
            (bit(value, high) << 1) | bit(value, low)
        }
        const fn pack_10bit(msb: u8, lsb: u16) -> u16 {
            ((msb as u16) << 2) | (lsb & 0x03)
        }
        let rg = raw[0];
        let bw = raw[1];
        Self {
            red: Coord {
                x: pack_10bit(raw[2], pair(rg, 0b1000_0000, 0b0100_0000)),
                y: pack_10bit(raw[3], pair(rg, 0b0010_0000, 0b0001_0000)),
            },
            green: Coord {
                x: pack_10bit(raw[4], pair(rg, 0b0000_1000, 0b0000_0100)),
                y: pack_10bit(raw[5], pair(rg, 0b0000_0010, 0b0000_0001)),
            },
            blue: Coord {
                x: pack_10bit(raw[6], pair(bw, 0b1000_0000, 0b0100_0000)),
                y: pack_10bit(raw[7], pair(bw, 0b0010_0000, 0b0001_0000)),
            },
            white: Coord {
                x: pack_10bit(raw[8], pair(bw, 0b0000_1000, 0b0000_0100)),
                y: pack_10bit(raw[9], pair(bw, 0b0000_0010, 0b0000_0001)),
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
