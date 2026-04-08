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
use crate::common::{BLOCK_LEN, DESC_LEN, Validation};

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
    /// Parses descriptors from base block bytes.
    #[must_use]
    pub fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        let desc = &raw[DTD_OFF..DTD_OFF + (DESC_LEN * DTD_NUM)];

        // TODO: Make this more concise, no need to please const anymore.
        let mut i = 0;
        let mut modes = [None; DTD_NUM];
        while i < DTD_NUM {
            let offset = i * DESC_LEN;
            let Ok(chunk): Result<&[u8; DESC_LEN], _> = desc[offset..offset + DESC_LEN].try_into()
            else {
                break;
            };
            if chunk[0] == 0 && chunk[1] == 0 {
                if let Some(display) = MonitorDesc::parse(chunk) {
                    modes[i] = Some(Mode::Display(display));
                }
            } else if let Some(timing) = DetailedTiming::parse(chunk) {
                modes[i] = Some(Mode::Timing(timing));
            }
            i += 1;
        }

        Self { modes }
    }

    // TODO: Just return with the array (it is only 4 elements), then the caller can index to its heart content.
    #[must_use]
    pub const fn mode(&self, i: usize) -> Option<Mode> {
        if i < DTD_NUM { self.modes[i] } else { None }
    }

    /// Validates the descriptors.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        let mut v = Validation::new();
        let mut i = 0;
        while i < DTD_NUM {
            if let Some(mode) = self.modes[i] {
                match mode {
                    Mode::Timing(timing) => v = v.then(timing.validate()),
                    Mode::Display(display) => v = v.then(display.validate()),
                }
            }
            i += 1;
        }
        v
    }
}
