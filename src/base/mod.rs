//! EDID 1.4 base block parsing.
//!
//! The base block is the first 128 bytes of an EDID blob and contains
//! all essential display information including manufacturer data, display
//! parameters, color characteristics, and timing descriptors.

pub mod basic;
pub mod chroma;
pub mod descriptor;
pub mod established;
pub mod footer;
pub mod header;
pub mod std1;

pub mod descriptors;

use crate::common::{BLOCK_LEN, DESC_LEN, Validation, checksum_ok, slice};

/// EDID 1.4 Base Block Structure (128 bytes)
///
/// | Bytes | Description |
/// | :--- | :--- |
/// | 0–19 | Header information |
/// | 20–24 | Basic display parameters |
/// | 25–34 | Chromaticity coordinates |
/// | 35–37 | Established timing bitmap |
/// | 38–53 | Standard timing information |
/// | 54–125 | Display timing descriptor followed by display/monitor descriptors |
/// | 126–127 | Extension flag and checksum |
///
/// # References
/// - [Wikipedia: EDID 1.4 Structure](https://en.wikipedia.org/wiki/Extended_Display_Identification_Data#Structure,_version_1.4)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BaseEdid {
    // TODO: Actually, we should only keep the raw bytes
    // Return parsing error on unrecoverable error: invalid header pattern
    // Then, parse at the getter scope
    header: header::Header,
    basic: basic::Basic,
    chroma: chroma::Chroma,
    established: established::Established,
    timings: std1::Std1,
    descriptors: descriptors::Descriptors,
    footer: footer::Footer,
}

impl BaseEdid {
    /// Parses the base block.
    /// Returns [`BaseEdid`].
    #[must_use]
    pub const fn parse(raw: &[u8; BLOCK_LEN]) -> Option<Self> {
        use basic::{BASIC_LEN, BASIC_OFF};
        use chroma::{CHROMA_LEN, CHROMA_OFF};
        use descriptors::{DTD_NUM, DTD_OFF};
        use established::{ESTABLISHED_LEN, ESTABLISHED_OFF};
        use footer::{FOOTER_LEN, FOOTER_OFF};
        use header::{HEADER_LEN, HEADER_OFF};
        use std1::{STANDARD_LEN, STANDARD_OFF};

        let header: [u8; HEADER_LEN] = slice(raw, HEADER_OFF);
        let Some(header) = header::Header::parse(&header) else {
            return None;
        };
        let basic: [u8; BASIC_LEN] = slice(raw, BASIC_OFF);
        let chroma: [u8; CHROMA_LEN] = slice(raw, CHROMA_OFF);
        let established: [u8; ESTABLISHED_LEN] = slice(raw, ESTABLISHED_OFF);
        let std1: [u8; STANDARD_LEN] = slice(raw, STANDARD_OFF);
        let descriptors: [u8; DTD_NUM * DESC_LEN] = slice(raw, DTD_OFF);
        let footer: [u8; FOOTER_LEN] = slice(raw, FOOTER_OFF);
        Some(Self {
            header,
            basic: basic::Basic::parse(&basic),
            chroma: chroma::Chroma::parse(&chroma),
            established: established::Established::parse(&established),
            timings: std1::Std1::parse(&std1),
            descriptors: descriptors::Descriptors::parse(&descriptors),
            footer: footer::Footer::parse(&footer),
        })
    }

    /// Header section.
    /// Returns [`Header`].
    #[must_use]
    pub const fn header(&self) -> header::Header {
        self.header
    }

    /// Basic display parameters.
    /// Returns [`Basic`].
    #[must_use]
    pub const fn basic(&self) -> basic::Basic {
        self.basic
    }

    /// Chromaticity coordinates.
    /// Returns [`Chroma`].
    #[must_use]
    pub const fn chroma(&self) -> chroma::Chroma {
        self.chroma
    }

    /// Established timings.
    /// Returns [`Established`].
    #[must_use]
    pub const fn established(&self) -> established::Established {
        self.established
    }

    /// Standard timings.
    /// Returns [`Std1`].
    #[must_use]
    pub const fn timings(&self) -> std1::Std1 {
        self.timings
    }

    /// Detailed timing and display descriptors.
    /// Returns [`Descriptors`].
    #[must_use]
    pub const fn descriptors(&self) -> descriptors::Descriptors {
        self.descriptors
    }

    /// Extension count and checksum.
    /// Returns [`Footer`].
    #[must_use]
    pub const fn footer(&self) -> footer::Footer {
        self.footer
    }

    /// Validate base block, collecting errors and warnings.
    #[must_use]
    pub fn validate(&self, raw: &[u8; BLOCK_LEN]) -> Validation {
        Validation::new()
            .then(self.header.validate())
            .err_if(!checksum_ok(raw), "Invalid base checksum")
    }
}
