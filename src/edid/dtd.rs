//! Detailed Timing Descriptors (DTDs) and Monitor Descriptors (bytes 54–125).
//!
//! Four 18-byte descriptors provide detailed timing information or
//! monitor metadata (serial number, name, range limits, color data, etc.).
//!
//! # Structure
//!
//! Each descriptor is 18 bytes. If bytes 0–1 are both zero, the descriptor
//! is a monitor descriptor; otherwise it's a detailed timing descriptor.
//!
//! | Offset | Count | Description |
//! |--------|-------|-------------|
//! | 54     | 4×18  | Detailed timing / monitor descriptors |

use crate::edid::descriptor::DESC_LEN;
use crate::edid::descriptor::monitor::MonitorDesc;
use crate::edid::descriptor::timing::DetailedTiming;

pub const DTD_OFF: usize = 54;
pub const DTD_NUM: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Timing(DetailedTiming),
    Display(MonitorDesc),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Descriptors {
    modes: [Option<Mode>; DTD_NUM],
}

impl Descriptors {
    #[must_use]
    pub fn parse(raw: &[u8; DTD_NUM * DESC_LEN]) -> Self {
        // TODO: Create the descriptors immutably
        let mut modes = [None; DTD_NUM];
        let chunks = raw.chunks_exact(DESC_LEN);
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
