use crate::edid::std1::{Timing, parse_timing_bytes};

const DESC_LEN: usize = 18;
const MODE_NUM: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Std2 {
    modes: [Option<Timing>; MODE_NUM],
    pad: u8,
}

impl Std2 {
    #[must_use]
    pub const fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        if raw[0] != 0 || raw[1] != 0 || raw[2] != 0 || raw[3] != 0xFA || raw[4] != 0 {
            return None;
        }
        let mut modes = [None; MODE_NUM];
        let mut i = 0;
        while i < MODE_NUM {
            let x_byte = raw[5 + i * 2];
            let y_byte = raw[6 + i * 2];
            modes[i] = parse_timing_bytes(x_byte, y_byte);
            i += 1;
        }
        Some(Self {
            modes,
            pad: raw[17],
        })
    }

    #[must_use]
    pub const fn mode(&self, i: usize) -> Option<Timing> {
        if i < MODE_NUM { self.modes[i] } else { None }
    }

    #[must_use]
    pub const fn pad(&self) -> u8 {
        self.pad
    }
}
