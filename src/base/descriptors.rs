//! Display timing descriptor followed by display/monitor descriptors (bytes 54–125).
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

use super::descriptor::monitor::MonitorDesc;
use super::descriptor::timing::DetailedTiming;
use crate::common::{DESC_LEN, Validation, slice};

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
    pub const fn parse(raw: &[u8; DTD_NUM * DESC_LEN]) -> Self {
        let mut modes = [None; DTD_NUM];
        let mut i = 0;

        while i < DTD_NUM {
            let offset = i * DESC_LEN;
            let chunk: [u8; DESC_LEN] = slice(raw, offset);

            if chunk[0] == 0 && chunk[1] == 0 {
                if let Some(display) = MonitorDesc::parse(&chunk) {
                    modes[i] = Some(Mode::Display(display));
                }
            } else if let Some(timing) = DetailedTiming::parse(&chunk) {
                modes[i] = Some(Mode::Timing(timing));
            }
            i += 1;
        }

        Self { modes }
    }

    #[must_use]
    pub const fn mode(&self, i: usize) -> Option<Mode> {
        if i < DTD_NUM { self.modes[i] } else { None }
    }

    /// Validates the descriptors.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        todo!()
    }
}
