use crate::edid::base::BASE_LEN;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    magic: [u8; 8],
    maker: [char; 3],
    product: u16,
    serial: u32,
    week: u8,
    year: u16,
    major: u8,
    minor: u8,
}

impl Header {
    #[must_use]
    pub fn parse(raw: &[u8; BASE_LEN]) -> Self {
        let maker_raw = u16::from_be_bytes([raw[8], raw[9]]);
        Self {
            magic: raw[0..8].try_into().unwrap_or_else(|_| unreachable!()),
            maker: [
                maker_char((maker_raw >> 10) & 0x1F),
                maker_char((maker_raw >> 5) & 0x1F),
                maker_char(maker_raw & 0x1F),
            ],
            product: u16::from_le_bytes([raw[10], raw[11]]),
            serial: u32::from_le_bytes([raw[12], raw[13], raw[14], raw[15]]),
            week: raw[16],
            year: 1990 + u16::from(raw[17]),
            major: raw[18],
            minor: raw[19],
        }
    }

    #[must_use]
    pub const fn magic(&self) -> [u8; 8] {
        self.magic
    }

    #[must_use]
    pub const fn maker(&self) -> [char; 3] {
        self.maker
    }

    #[must_use]
    pub const fn product(&self) -> u16 {
        self.product
    }

    #[must_use]
    pub const fn serial(&self) -> u32 {
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
    pub const fn major(&self) -> u8 {
        self.major
    }

    #[must_use]
    pub const fn minor(&self) -> u8 {
        self.minor
    }
}

fn maker_char(raw: u16) -> char {
    if (1..=26).contains(&raw) {
        (u8::try_from(raw).unwrap_or_else(|_| unreachable!()) + b'@') as char
    } else {
        '?'
    }
}
