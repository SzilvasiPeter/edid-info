use crate::edid::descriptor::monitor::MonitorDesc;
use crate::edid::descriptor::timing::DetailedTiming;
use crate::edid::{BLOCK_LEN, DESC_LEN};

pub const DTD_OFF: usize = 54;
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
    pub fn parse_base(raw: &[u8; BLOCK_LEN]) -> Self {
        let mut modes = [None; DTD_NUM];

        let chunks = raw[DTD_OFF..DTD_OFF + DTD_NUM * DESC_LEN].chunks_exact(DESC_LEN);
        for (mode, chunk) in modes.iter_mut().zip(chunks) {
            if let Ok(desc_raw) = chunk.try_into() {
                *mode = DetailedTiming::parse(desc_raw)
                    .map(Mode::Timing)
                    .or_else(|| MonitorDesc::parse(desc_raw).map(Mode::Display));
            }
        }

        Self { modes }
    }

    #[must_use]
    pub const fn mode(&self, i: usize) -> Option<Mode> {
        if i < DTD_NUM { self.modes[i] } else { None }
    }
}
