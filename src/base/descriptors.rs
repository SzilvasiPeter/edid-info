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

use super::descriptor::dtd::DetailedTiming;
use super::descriptor::monitor::Monitor;
use crate::base::descriptor::monitor::DisplayDescriptor;
use crate::common::{BLOCK_LEN, DESC_LEN, FailureKind, Validation, WarningKind};

pub const DESCRIPTORS_OFF: usize = 54;
pub const DESC_NUM: usize = 4;
pub const DTD_SIZE: usize = DESC_NUM * DESC_LEN;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Descriptor {
    Timing(DetailedTiming),
    Display(Monitor),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Descriptors {
    bytes: [u8; DTD_SIZE],
    legacy: bool,
}

impl Descriptors {
    /// Parses descriptors from base block bytes.
    #[must_use]
    pub fn new(raw: &[u8; BLOCK_LEN], legacy: bool) -> Self {
        let mut bytes = [0u8; DTD_SIZE];
        bytes.copy_from_slice(&raw[DESCRIPTORS_OFF..DESCRIPTORS_OFF + DTD_SIZE]);
        Self { bytes, legacy }
    }

    /// Returns an iterator over the parsed descriptors.
    pub fn iter(&self) -> impl Iterator<Item = Option<Descriptor>> {
        let (chunks, remainder) = self.bytes.as_chunks::<DESC_LEN>();
        debug_assert!(remainder.is_empty());

        chunks.iter().map(move |chunk| match (chunk[0], chunk[1]) {
            (0, 0) => Some(Descriptor::Display(Monitor::new(chunk, self.legacy))),
            _ => Some(Descriptor::Timing(DetailedTiming::new(chunk))),
        })
    }

    /// Validates the descriptors.
    #[must_use]
    pub fn validate(&self, continous_frequency: bool) -> Validation {
        let mut validation = Validation::new();
        validation = validation.fail_if(
            !matches!(self.iter().nth(0), Some(Some(Descriptor::Timing(_)))),
            FailureKind::FirstDescriptorNotDetailedTiming,
        );
        validation = validation.fail_if(
            self.iter().any(|d| d.is_none()),
            FailureKind::InvalidDescriptor,
        );
        validation = validation.fail_if(
            self.iter()
                .flatten()
                .skip_while(|d| matches!(d, Descriptor::Timing(_)))
                .any(|d| matches!(d, Descriptor::Timing(_))),
            FailureKind::InvalidDescriptorOrder,
        );

        let has_range = self.iter().flatten().any(|d| match d {
            Descriptor::Display(m) => {
                matches!(m.descriptor(), DisplayDescriptor::RangeLimits(_))
            }
            Descriptor::Timing(_) => false,
        });
        validation = validation.warn_if(
            continous_frequency && !has_range,
            WarningKind::RangeLimitsRequired,
        );

        let has_monitor_name = self.iter().flatten().any(|d| match d {
            Descriptor::Display(m) => {
                matches!(m.descriptor(), DisplayDescriptor::MonitorName(_))
            }
            Descriptor::Timing(_) => false,
        });
        validation = validation.warn_if(!has_monitor_name, WarningKind::MissingMonitorName);

        for descriptor in self.iter().flatten() {
            match descriptor {
                Descriptor::Timing(timing) => validation = validation.then(timing.validate()),
                Descriptor::Display(display) => validation = validation.then(display.validate()),
            }
        }
        validation
    }
}
