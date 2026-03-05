use crate::edid::base::BASE_LEN;
use crate::edid::descriptor::monitor::MonitorDesc;
use crate::edid::descriptor::timing::{DETAILED_LEN, DetailedTiming};

pub const DTD_OFF: usize = 54;
pub const DTD_LEN: usize = 18;
pub const DTD_NUM: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Timing(DetailedTiming),
    Display(MonitorDesc),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dtd {
    modes: [Option<Mode>; DTD_NUM],
}

impl Dtd {
    #[must_use]
    pub fn parse_base(raw: &[u8; BASE_LEN]) -> Self {
        let mut modes = [None; DTD_NUM];

        let mut i = 0;
        while i < DTD_NUM {
            let off = DTD_OFF + i * DTD_LEN;
            let mut desc_raw = [0; DETAILED_LEN];
            desc_raw.copy_from_slice(&raw[off..off + DTD_LEN]);
            modes[i] = DetailedTiming::parse(&desc_raw).map_or_else(
                || MonitorDesc::parse(&desc_raw).map(Mode::Display),
                |timing| Some(Mode::Timing(timing)),
            );
            i += 1;
        }

        Self { modes }
    }

    #[must_use]
    pub const fn mode(&self, i: usize) -> Option<Mode> {
        if i < DTD_NUM { self.modes[i] } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::{DTD_LEN, DTD_OFF, Dtd, Mode};

    #[test]
    fn parse_timing_in_second_slot() {
        let mut raw = [0_u8; 128];
        let off = DTD_OFF + DTD_LEN;
        raw[off] = 0x01;
        raw[off + 1] = 0x1d;

        let out = Dtd::parse_base(&raw);
        match out.mode(1) {
            Some(Mode::Timing(timing)) => assert_eq!(timing.pixel_clock_hz(), 74_250_000),
            _ => panic!("slot 1 should parse as timing"),
        }
    }
}
