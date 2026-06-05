//! Monitor Descriptor parsing.
//!
//! Monitor descriptors provide display metadata such as serial number,
//! name, range limits, and color characteristics. Identified by
//! bytes 0–2 being zero.

use super::cvt3::Cvt3;
use super::dcm::Color;
use super::established::EstablishedTimings;
use super::range::DisplayRangeLimits;
use super::standard::StandardTimings;
use super::white_point::WhitePoint;
use crate::common::{DESC_LEN, Validation};

/// A descriptor containing ASCII text.
///
/// Used for Serial Number, Monitor Name, and other text fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DescriptorString {
    raw: [u8; 13],
}

impl DescriptorString {
    /// Returns the text as a string slice, trimmed of trailing whitespace and nulls.
    #[must_use]
    pub fn text(&self) -> &str {
        core::str::from_utf8(&self.raw)
            .unwrap_or("")
            .trim_end_matches(['\0', '\n', ' '])
    }
}

// TODO: Add documentation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayDescriptor {
    SerialNumber(DescriptorString),
    Text(DescriptorString),
    RangeLimits(DisplayRangeLimits),
    MonitorName(DescriptorString),
    WhitePoint(WhitePoint),
    StdTimings(StandardTimings),
    Dcm(Color),
    Cvt3Byte(Cvt3),
    EstablishedTiming3(EstablishedTimings),
    Dummy,
    VendorReserved([u8; DESC_LEN]),
    Unknown([u8; DESC_LEN]),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Monitor {
    raw: [u8; DESC_LEN],
    legacy: bool,
}

impl Monitor {
    #[must_use]
    pub const fn new(raw: &[u8; DESC_LEN], legacy: bool) -> Self {
        Self { raw: *raw, legacy }
    }

    #[must_use]
    pub fn descriptor(&self) -> DisplayDescriptor {
        let mut data = [0u8; 13];
        data.copy_from_slice(&self.raw[5..DESC_LEN]);
        let text = DescriptorString { raw: data };

        match self.raw[3] {
            0xFF => DisplayDescriptor::SerialNumber(text),
            0xFE => DisplayDescriptor::Text(text),
            0xFD => DisplayDescriptor::RangeLimits(DisplayRangeLimits::parse(&self.raw)),
            0xFC => DisplayDescriptor::MonitorName(text),
            0xFB => DisplayDescriptor::WhitePoint(WhitePoint::parse(&self.raw)),
            0xFA => DisplayDescriptor::StdTimings(StandardTimings::new(&self.raw, self.legacy)),
            0xF9 => DisplayDescriptor::Dcm(Color::parse(&self.raw)),
            0xF8 => DisplayDescriptor::Cvt3Byte(Cvt3::parse(&self.raw)),
            0xF7 => DisplayDescriptor::EstablishedTiming3(EstablishedTimings::parse(&self.raw)),
            0x10 => DisplayDescriptor::Dummy,
            0x00..=0x0F => DisplayDescriptor::VendorReserved(self.raw),
            _ => DisplayDescriptor::Unknown(self.raw),
        }
    }

    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new()
    }
}
