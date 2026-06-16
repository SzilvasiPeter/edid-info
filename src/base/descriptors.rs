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
use crate::base::descriptor::range::VideoTimingSupport;
use crate::common::{BLOCK_LEN, DESC_LEN, FailureKind, Validation, WarningKind};

/// Byte offset of the first descriptor block in the EDID base block.
pub const DESCRIPTORS_OFF: usize = 54;
/// Number of descriptor blocks in the EDID base block.
pub const DESC_NUM: usize = 4;
/// Total size of all descriptor blocks in bytes.
pub const DTD_SIZE: usize = DESC_NUM * DESC_LEN;

/// A descriptor which can either be a Detailed Timing Descriptor (DTD) or a Monitor Display Descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Descriptor {
    /// Detailed Timing Descriptor (DTD).
    Timing(DetailedTiming),
    /// Monitor Display Descriptor.
    Display(Monitor),
}

/// Collection of the four descriptor blocks in the EDID base block.
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
    pub fn iter(&self) -> impl Iterator<Item = Descriptor> {
        let (chunks, remainder) = self.bytes.as_chunks::<DESC_LEN>();
        debug_assert!(remainder.is_empty());

        chunks.iter().map(move |chunk| match (chunk[0], chunk[1]) {
            (0, 0) => Descriptor::Display(Monitor::new(chunk, self.legacy)),
            _ => Descriptor::Timing(DetailedTiming::new(chunk)),
        })
    }

    /// Validates the descriptors.
    #[must_use]
    pub fn validate(&self, cont_freq: bool) -> Validation {
        let mut validation = Validation::new();
        validation = validation.fail_if(
            !matches!(self.iter().nth(0), Some(Descriptor::Timing(_))),
            FailureKind::FirstDescriptorNotDetailedTiming,
        );
        validation = validation.fail_if(
            self.iter()
                .skip_while(|d| matches!(d, Descriptor::Timing(_)))
                .any(|d| matches!(d, Descriptor::Timing(_))),
            FailureKind::InvalidDescriptorOrder,
        );

        let has_undefined = self.iter().any(|d| match d {
            Descriptor::Display(m) => {
                matches!(m.descriptor(), DisplayDescriptor::Undefined(_))
            }
            Descriptor::Timing(_) => false,
        });
        validation = validation.fail_if(has_undefined, FailureKind::UndefinedDescriptor);

        let has_gtf_or_cvt = self.iter().any(|d| {
            let Descriptor::Display(m) = d else {
                return false;
            };
            let DisplayDescriptor::RangeLimits(r) = m.descriptor() else {
                return false;
            };
            matches!(
                r.timing(),
                VideoTimingSupport::DefaultGtf
                    | VideoTimingSupport::SecondaryGtf(_)
                    | VideoTimingSupport::Cvt(_)
            )
        });
        validation = validation.fail_if(
            has_gtf_or_cvt && !cont_freq,
            FailureKind::GtfAndCvtRequiresContFreq,
        );

        let has_range = self.iter().any(|d| match d {
            Descriptor::Display(m) => {
                matches!(m.descriptor(), DisplayDescriptor::RangeLimits(_))
            }
            Descriptor::Timing(_) => false,
        });
        validation = validation.warn_if(cont_freq && !has_range, WarningKind::RangeLimitsRequired);

        let has_monitor_name = self.iter().any(|d| match d {
            Descriptor::Display(m) => {
                matches!(m.descriptor(), DisplayDescriptor::ProductName(_))
            }
            Descriptor::Timing(_) => false,
        });
        validation = validation.warn_if(!has_monitor_name, WarningKind::MissingMonitorName);

        for descriptor in self.iter() {
            match descriptor {
                Descriptor::Timing(t) => validation = validation.then(t.validate()),
                Descriptor::Display(d) => validation = validation.then(d.validate()),
            }
        }
        validation
    }
}
