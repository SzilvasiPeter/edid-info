pub const CTA_LEN: usize = 128;
pub const CTA_TAG: u8 = 0b0000_0010;

/// CTA Extension Block header structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    tag: u8,
    rev: u8,
    dtd_off: u8,
    flags: u8,
}

impl Header {
    #[must_use]
    pub fn parse(raw: &[u8; CTA_LEN]) -> Option<Self> {
        (raw[0] == CTA_TAG).then_some(Self {
            tag: raw[0],
            rev: raw[1],
            dtd_off: raw[2],
            flags: raw[3],
        })
    }

    #[must_use]
    pub const fn tag(&self) -> u8 {
        self.tag
    }

    #[must_use]
    pub const fn rev(&self) -> u8 {
        self.rev
    }

    #[must_use]
    pub const fn dtd_off(&self) -> u8 {
        self.dtd_off
    }

    #[must_use]
    pub const fn underscan(&self) -> bool {
        (self.flags & 0b1000_0000) != 0
    }

    #[must_use]
    pub const fn basic_audio(&self) -> bool {
        (self.flags & 0b0100_0000) != 0
    }

    #[must_use]
    pub const fn ycbcr_444(&self) -> bool {
        (self.flags & 0b0010_0000) != 0
    }

    #[must_use]
    pub const fn ycbcr_422(&self) -> bool {
        (self.flags & 0b0001_0000) != 0
    }

    #[must_use]
    pub const fn native_dtd_num(&self) -> u8 {
        self.flags & 0b0000_1111
    }

    #[must_use]
    pub const fn data_block_end(&self) -> usize {
        if self.dtd_off == 0 {
            127
        } else if self.dtd_off >= 4 {
            self.dtd_off as usize
        } else {
            4
        }
    }

    #[must_use]
    pub const fn dtd_start(&self) -> Option<usize> {
        if self.dtd_off >= 4 && self.dtd_off < 127 {
            Some(self.dtd_off as usize)
        } else {
            None
        }
    }
}
