//! Monitor Descriptor parsing.
//!
//! Monitor descriptors provide display metadata such as serial number,
//! name, range limits, and color characteristics. Identified by bytes 0–1 being zero.

use super::color::ColorPoint;
use super::cvt3::Cvt3;
use super::dcm::Dcm;
use super::established::EstTimings;
use super::range::RangeLimits;
use super::standard::StdTimings;
use crate::common::{DESC_LEN, FailureKind, Validation, WarningKind};

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
    /// Tag 0x10: Dummy Descriptor.
    Dummy,
    /// Tags 0x00–0x0F: Manufacturer Specified Display Descriptors.
    VendorReserved([u8; 13]),
    /// Tags 0x11–0xF6: Reserved / Undefined.
    Undefined([u8; 13]),
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
            0xFD => DisplayDescriptor::RangeLimits(RangeLimits::parse(self.raw[4], &payload)),
            0xFC => DisplayDescriptor::ProductName(text),
            0xFB => DisplayDescriptor::ColorPoint(ColorPoint::parse(&payload)),
            0xFA => DisplayDescriptor::StdTimings(StdTimings::new(&payload, self.legacy)),
            0xF9 => DisplayDescriptor::Dcm(Dcm::parse(&payload)),
            0xF8 => DisplayDescriptor::Cvt3(Cvt3::parse(&payload)),
            0xF7 => DisplayDescriptor::EstTimings(EstTimings::parse(&payload)),
            0x10 => DisplayDescriptor::Dummy,
            0x00..=0x0F => DisplayDescriptor::VendorReserved(payload),
            0x11..=0xF6 => DisplayDescriptor::Undefined(payload),
        }
    }

    /// Validates the monitor descriptor according to the VESA specification.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let mut validation = Validation::new();
        validation = validation.fail_if(
            self.raw.iter().all(|&b| b == 0),
            FailureKind::AllZeroDescriptor,
        );
        validation =
            validation.fail_if(self.raw[2] != 0, FailureKind::MonitorReservedByteIsNonZero);
        validation = validation.fail_if(
            self.raw[3] != 0xFD && self.raw[4] != 0,
            FailureKind::MonitorReservedByteIsNonZero,
        );
        if self.raw[3] == 0xFD {
            let is_gtf = self.raw[10] == 0x00 || self.raw[10] == 0x02;
            validation = validation.warn_if(is_gtf, WarningKind::GtfIsDeprecated);
        }

        validation
    }
}
