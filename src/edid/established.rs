use crate::edid::BLOCK_LEN;

pub const ESTABLISHED_OFF: usize = 35;
pub const ESTABLISHED_LEN: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Established {
    raw: [u8; ESTABLISHED_LEN],
}

#[rustfmt::skip]
impl Established {
    #[must_use]
    pub fn parse_base(raw: &[u8; BLOCK_LEN]) -> Self {
        let mut out = [0; ESTABLISHED_LEN];
        out.copy_from_slice(&raw[ESTABLISHED_OFF..ESTABLISHED_OFF + ESTABLISHED_LEN]);
        Self::parse(&out)
    }

    #[must_use]
    pub const fn parse(raw: &[u8; ESTABLISHED_LEN]) -> Self {
        Self { raw: *raw }
    }

    #[must_use]
    pub const fn t_720_400_70(&self) -> bool { (self.raw[0] & 0x80) != 0 }
    #[must_use]
    pub const fn t_720_400_88(&self) -> bool { (self.raw[0] & 0x40) != 0 }
    #[must_use]
    pub const fn t_640_480_60(&self) -> bool { (self.raw[0] & 0x20) != 0 }
    #[must_use]
    pub const fn t_640_480_67(&self) -> bool { (self.raw[0] & 0x10) != 0 }
    #[must_use]
    pub const fn t_640_480_72(&self) -> bool { (self.raw[0] & 0x08) != 0 }
    #[must_use]
    pub const fn t_640_480_75(&self) -> bool { (self.raw[0] & 0x04) != 0 }
    #[must_use]
    pub const fn t_800_600_56(&self) -> bool { (self.raw[0] & 0x02) != 0 }
    #[must_use]
    pub const fn t_800_600_60(&self) -> bool { (self.raw[0] & 0x01) != 0 }
    #[must_use]
    pub const fn t_800_600_72(&self) -> bool { (self.raw[1] & 0x80) != 0 }
    #[must_use]
    pub const fn t_800_600_75(&self) -> bool { (self.raw[1] & 0x40) != 0 }
    #[must_use]
    pub const fn t_832_624_75(&self) -> bool { (self.raw[1] & 0x20) != 0 }
    #[must_use]
    pub const fn t_1024_768_87i(&self) -> bool { (self.raw[1] & 0x10) != 0 }
    #[must_use]
    pub const fn t_1024_768_60(&self) -> bool { (self.raw[1] & 0x08) != 0 }
    #[must_use]
    pub const fn t_1024_768_70(&self) -> bool { (self.raw[1] & 0x04) != 0 }
    #[must_use]
    pub const fn t_1024_768_75(&self) -> bool { (self.raw[1] & 0x02) != 0 }
    #[must_use]
    pub const fn t_1280_1024_75(&self) -> bool { (self.raw[1] & 0x01) != 0 }
    #[must_use]
    pub const fn t_1152_870_75(&self) -> bool { (self.raw[2] & 0x80) != 0 }
    #[must_use]
    pub const fn manufacturer_bits(&self) -> u8 { self.raw[2] & 0x7F }
}
