const DESC_LEN: usize = 18;
const VERSION: u8 = 0x01;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Aspect {
    A4_3,
    A16_9,
    A16_10,
    A15_9,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrefRate {
    Hz50,
    Hz60,
    Hz75,
    Hz85,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[expect(clippy::struct_excessive_bools, reason = "Spec-aligned EDID bitfields")]
pub struct Mode {
    addr_lines: u16,
    aspect: Aspect,
    pref: PrefRate,
    hz50: bool,
    hz60: bool,
    hz75: bool,
    hz85: bool,
    hz60_rb: bool,
}

impl Mode {
    fn parse(raw: [u8; 3]) -> Self {
        let addr_lines = (u16::from(raw[0])) | (u16::from((raw[1] >> 4) & 0x0F) << 8);
        let aspect = match (raw[1] >> 2) & 0b11 {
            0b00 => Aspect::A4_3,
            0b01 => Aspect::A16_9,
            0b10 => Aspect::A16_10,
            _ => Aspect::A15_9,
        };
        let pref = match (raw[2] >> 5) & 0b11 {
            0b00 => PrefRate::Hz50,
            0b01 => PrefRate::Hz60,
            0b10 => PrefRate::Hz75,
            _ => PrefRate::Hz85,
        };
        Self {
            addr_lines,
            aspect,
            pref,
            hz50: (raw[2] & 0b0001_0000) != 0,
            hz60: (raw[2] & 0b0000_1000) != 0,
            hz75: (raw[2] & 0b0000_0100) != 0,
            hz85: (raw[2] & 0b0000_0010) != 0,
            hz60_rb: (raw[2] & 0b0000_0001) != 0,
        }
    }

    #[must_use]
    pub const fn addr_lines(&self) -> u16 {
        self.addr_lines
    }
    #[must_use]
    pub const fn aspect(&self) -> Aspect {
        self.aspect
    }
    #[must_use]
    pub const fn pref(&self) -> PrefRate {
        self.pref
    }
    #[must_use]
    pub const fn hz50(&self) -> bool {
        self.hz50
    }
    #[must_use]
    pub const fn hz60(&self) -> bool {
        self.hz60
    }
    #[must_use]
    pub const fn hz75(&self) -> bool {
        self.hz75
    }
    #[must_use]
    pub const fn hz85(&self) -> bool {
        self.hz85
    }
    #[must_use]
    pub const fn hz60_rb(&self) -> bool {
        self.hz60_rb
    }

    #[must_use]
    pub const fn v_lines(&self) -> u16 {
        (self.addr_lines + 1) * 2
    }

    #[must_use]
    pub const fn h_pixels(&self) -> u16 {
        let v = self.v_lines();
        let h = match self.aspect {
            Aspect::A4_3 => v * 4 / 3,
            Aspect::A16_9 => v * 16 / 9,
            Aspect::A16_10 => v * 16 / 10,
            Aspect::A15_9 => v * 15 / 9,
        };
        (h / 8) * 8
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cvt3 {
    mode1: Mode,
    mode2: Mode,
    mode3: Mode,
    mode4: Mode,
}

impl Cvt3 {
    #[must_use]
    pub fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        if raw[0] != 0 || raw[1] != 0 || raw[2] != 0 || raw[3] != 0xF8 || raw[4] != 0 {
            return None;
        }
        if raw[5] != VERSION {
            return None;
        }
        Some(Self {
            mode1: Mode::parse([raw[6], raw[7], raw[8]]),
            mode2: Mode::parse([raw[9], raw[10], raw[11]]),
            mode3: Mode::parse([raw[12], raw[13], raw[14]]),
            mode4: Mode::parse([raw[15], raw[16], raw[17]]),
        })
    }

    #[must_use]
    pub const fn mode1(&self) -> Mode {
        self.mode1
    }
    #[must_use]
    pub const fn mode2(&self) -> Mode {
        self.mode2
    }
    #[must_use]
    pub const fn mode3(&self) -> Mode {
        self.mode3
    }
    #[must_use]
    pub const fn mode4(&self) -> Mode {
        self.mode4
    }
}
