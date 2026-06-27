//! Monitor Descriptor parsing.
//!
//! Monitor descriptors provide display metadata such as serial number,
//! name, range limits, and color characteristics. Identified by bytes 0–1 being zero.
//!
//! # Examples
//!
//! ```rust
//! use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};
//! use edid_info::common::DESC_LEN;
//!
//! // Product name descriptor (tag 0xFC)
//! let mut raw = [0u8; DESC_LEN];
//! raw[3] = 0xFC;
//! raw[5..11].copy_from_slice(b"My Mon");
//!
//! let monitor = Monitor::new(&raw, false);
//! if let DisplayDescriptor::ProductName(name) = monitor.descriptor() {
//!     assert_eq!(name.text(), "My Mon");
//! } else {
//!     panic!("expected ProductName descriptor");
//! }
//! assert!(monitor.validate().is_valid());
//! ```

use super::color::ColorPoint;
use super::cvt3::Cvt3;
use super::dcm::Dcm;
use super::established::EstTimings;
use super::range::RangeLimits;
use super::standard::StdTimings;
use crate::common::{DESC_LEN, FailureKind, Validation, WarningKind};

/// Display metadata descriptors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayDescriptor {
    /// Tag 0xFF: Display Product Serial Number.
    SerialNumber(DescriptorString),
    /// Tag 0xFE: Alphanumeric Data String (ASCII).
    DataString(DescriptorString),
    /// Tag 0xFD: Display Range Limits (GTF/CVT timing parameters).
    RangeLimits(RangeLimits),
    /// Tag 0xFC: Display Product Name.
    ProductName(DescriptorString),
    /// Tag 0xFB: Color Point Data (white point coordinates).
    ColorPoint(ColorPoint),
    /// Tag 0xFA: Standard Timing Identifications.
    StdTimings(StdTimings),
    /// Tag 0xF9: Display Color Management (DCM) Data.
    Dcm(Dcm),
    /// Tag 0xF8: CVT 3 Byte Timing Codes.
    Cvt3(Cvt3),
    /// Tag 0xF7: Established Timings III.
    EstTimings(EstTimings),
    /// Tag 0x10: Dummy Descriptor (payload must be all zeroes).
    Dummy([u8; 13]),
    /// Tags 0x00–0x0F: Manufacturer Specified Display Descriptors.
    VendorReserved([u8; 13]),
    /// Tags 0x11–0xF6: Reserved / Undefined.
    Undefined([u8; 13]),
}

impl DisplayDescriptor {
    /// Validates the specific inner metadata of the display descriptor.
    #[must_use]
    pub fn validate(&self) -> Validation {
        match self {
            Self::RangeLimits(d) => d.validate(),
            Self::ColorPoint(d) => d.validate(),
            Self::StdTimings(d) => d.validate(),
            Self::Dcm(d) => d.validate(),
            Self::Cvt3(d) => d.validate(),
            Self::EstTimings(d) => d.validate(),
            Self::Dummy(d) => {
                Validation::new().warn_if(d.iter().any(|&b| b != 0), WarningKind::DummyNonZeroBytes)
            }
            Self::SerialNumber(_)
            | Self::DataString(_)
            | Self::ProductName(_)
            | Self::VendorReserved(_)
            | Self::Undefined(_) => Validation::new(),
        }
    }
}

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

/// An 18-byte Monitor Descriptor parser.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Monitor {
    raw: [u8; DESC_LEN],
    legacy: bool,
}

impl Monitor {
    /// Creates a new `Monitor` descriptor parser.
    #[must_use]
    pub const fn new(raw: &[u8; DESC_LEN], legacy: bool) -> Self {
        Self { raw: *raw, legacy }
    }

    /// Parses and returns the inner display descriptor type and its associated data.
    #[must_use]
    pub fn descriptor(&self) -> DisplayDescriptor {
        let payload: [u8; 13] = self.raw[5..DESC_LEN].try_into().unwrap_or([0; 13]);
        let text = DescriptorString { raw: payload };

        match self.raw[3] {
            0xFF => DisplayDescriptor::SerialNumber(text),
            0xFE => DisplayDescriptor::DataString(text),
            0xFD => DisplayDescriptor::RangeLimits(RangeLimits::new(self.raw[4], &payload)),
            0xFC => DisplayDescriptor::ProductName(text),
            0xFB => DisplayDescriptor::ColorPoint(ColorPoint::new(&payload)),
            0xFA => DisplayDescriptor::StdTimings(StdTimings::new(&payload, self.legacy)),
            0xF9 => DisplayDescriptor::Dcm(Dcm::new(&payload)),
            0xF8 => DisplayDescriptor::Cvt3(Cvt3::new(&payload)),
            0xF7 => DisplayDescriptor::EstTimings(EstTimings::new(&payload)),
            0x10 => DisplayDescriptor::Dummy(payload),
            0x00..=0x0F => DisplayDescriptor::VendorReserved(payload),
            0x11..=0xF6 => DisplayDescriptor::Undefined(payload),
        }
    }

    /// Validates the monitor descriptor according to the VESA specification.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let all_zero = self.raw.iter().all(|&b| b == 0);
        let bad_reserved = self.raw[2] != 0 || (self.raw[3] != 0xFD && self.raw[4] != 0);
        Validation::new()
            .fail_if(all_zero, FailureKind::AllZeroDescriptor)
            .fail_if(bad_reserved, FailureKind::MonitorReservedByteIsNonZero)
            .then(self.descriptor().validate())
    }
}

#[cfg(test)]
mod tests {
    use super::{DisplayDescriptor, Monitor};
    use crate::common::{DESC_LEN, WarningKind};

    #[test]
    fn examples_docstring() {
        // Product name descriptor (tag 0xFC)
        let mut raw = [0u8; DESC_LEN];
        raw[3] = 0xFC;
        raw[5..11].copy_from_slice(b"My Mon");

        let monitor = Monitor::new(&raw, false);
        if let DisplayDescriptor::ProductName(name) = monitor.descriptor() {
            assert_eq!(name.text(), "My Mon");
        } else {
            panic!("expected ProductName descriptor");
        }
        assert!(monitor.validate().is_valid());
    }

    #[test]
    fn validate_dummy_rejects_nonzero_payload() {
        let mut raw = [0u8; 18];
        raw[3] = 0x10;
        raw[5] = 0x01;

        let monitor = Monitor::new(&raw, false);
        let v = monitor.validate();
        assert!(
            v.warnings & (1 << WarningKind::DummyNonZeroBytes as u8) != 0,
            "expected warning DummyNonZeroBytes",
        );
    }

    #[test]
    fn validate_dummy_accepts_zero_payload() {
        let mut raw = [0u8; 18];
        raw[3] = 0x10;

        let monitor = Monitor::new(&raw, false);
        let v = monitor.validate();
        assert!(v.is_valid());
    }
}
