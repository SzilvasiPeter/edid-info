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

        let mut timing_raw = [0; DETAILED_LEN];
        timing_raw.copy_from_slice(&raw[DTD_OFF..DTD_OFF + DTD_LEN]);
        modes[0] = DetailedTiming::parse(&timing_raw).map(Mode::Timing);

        let mut i = 1;
        while i < DTD_NUM {
            let off = DTD_OFF + i * DTD_LEN;
            let mut display_raw = [0; DTD_LEN];
            display_raw.copy_from_slice(&raw[off..off + DTD_LEN]);
            modes[i] = MonitorDesc::parse(&display_raw).map(Mode::Display);
            i += 1;
        }

        Self { modes }
    }

    #[must_use]
    pub const fn mode(&self, i: usize) -> Option<Mode> {
        if i < DTD_NUM { self.modes[i] } else { None }
    }
}
