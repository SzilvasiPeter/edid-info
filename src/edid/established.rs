use crate::edid::base::BASE_LEN;

pub const ESTABLISHED_OFF: usize = 35;
pub const ESTABLISHED_LEN: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Established {
    raw: [u8; ESTABLISHED_LEN],
}

impl Established {
    #[must_use]
    pub fn parse_base(raw: &[u8; BASE_LEN]) -> Self {
        let mut out = [0; ESTABLISHED_LEN];
        out.copy_from_slice(&raw[ESTABLISHED_OFF..ESTABLISHED_OFF + ESTABLISHED_LEN]);
        Self::parse(&out)
    }

    #[must_use]
    pub const fn parse(raw: &[u8; ESTABLISHED_LEN]) -> Self {
        Self { raw: *raw }
    }

    #[must_use]
    pub const fn t_720_400_70(&self) -> bool {
        (self.raw[0] & 0b1000_0000) != 0
    }

    #[must_use]
    pub const fn t_720_400_88(&self) -> bool {
        (self.raw[0] & 0b0100_0000) != 0
    }

    #[must_use]
    pub const fn t_640_480_60(&self) -> bool {
        (self.raw[0] & 0b0010_0000) != 0
    }

    #[must_use]
    pub const fn t_640_480_67(&self) -> bool {
        (self.raw[0] & 0b0001_0000) != 0
    }

    #[must_use]
    pub const fn t_640_480_72(&self) -> bool {
        (self.raw[0] & 0b0000_1000) != 0
    }

    #[must_use]
    pub const fn t_640_480_75(&self) -> bool {
        (self.raw[0] & 0b0000_0100) != 0
    }

    #[must_use]
    pub const fn t_800_600_56(&self) -> bool {
        (self.raw[0] & 0b0000_0010) != 0
    }

    #[must_use]
    pub const fn t_800_600_60(&self) -> bool {
        (self.raw[0] & 0b0000_0001) != 0
    }

    #[must_use]
    pub const fn t_800_600_72(&self) -> bool {
        (self.raw[1] & 0b1000_0000) != 0
    }

    #[must_use]
    pub const fn t_800_600_75(&self) -> bool {
        (self.raw[1] & 0b0100_0000) != 0
    }

    #[must_use]
    pub const fn t_832_624_75(&self) -> bool {
        (self.raw[1] & 0b0010_0000) != 0
    }

    #[must_use]
    pub const fn t_1024_768_87i(&self) -> bool {
        (self.raw[1] & 0b0001_0000) != 0
    }

    #[must_use]
    pub const fn t_1024_768_60(&self) -> bool {
        (self.raw[1] & 0b0000_1000) != 0
    }

    #[must_use]
    pub const fn t_1024_768_70(&self) -> bool {
        (self.raw[1] & 0b0000_0100) != 0
    }

    #[must_use]
    pub const fn t_1024_768_75(&self) -> bool {
        (self.raw[1] & 0b0000_0010) != 0
    }

    #[must_use]
    pub const fn t_1280_1024_75(&self) -> bool {
        (self.raw[1] & 0b0000_0001) != 0
    }

    #[must_use]
    pub const fn t_1152_870_75(&self) -> bool {
        (self.raw[2] & 0b1000_0000) != 0
    }

    #[must_use]
    pub const fn manufacturer_bits(&self) -> u8 {
        self.raw[2] & 0b0111_1111
    }
}
