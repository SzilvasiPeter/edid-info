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
use basic::{BASIC_LEN, BASIC_OFF};
use chroma::{CHROMA_LEN, CHROMA_OFF};
use descriptors::{DTD_NUM, DTD_OFF};
use established::{ESTABLISHED_LEN, ESTABLISHED_OFF};
use footer::{FOOTER_LEN, FOOTER_OFF};
use header::{HEADER_LEN, HEADER_OFF};
use std1::{STANDARD_LEN, STANDARD_OFF};

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
pub struct Base {
    raw: [u8; BLOCK_LEN],
}

impl Base {
    #[must_use]
    pub const fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        Self { raw: *raw }
    }

    #[must_use]
    pub const fn header(&self) -> header::Header {
        let header: [u8; HEADER_LEN] = slice(&self.raw, HEADER_OFF);
        header::Header::new(&header)
    }

    #[must_use]
    pub const fn basic(&self) -> basic::Basic {
        let basic: [u8; BASIC_LEN] = slice(&self.raw, BASIC_OFF);
        basic::Basic::parse(&basic)
    }

    #[must_use]
    pub const fn chroma(&self) -> chroma::Chroma {
        let chroma: [u8; CHROMA_LEN] = slice(&self.raw, CHROMA_OFF);
        chroma::Chroma::parse(&chroma)
    }

    #[must_use]
    pub const fn established(&self) -> established::Established {
        let established: [u8; ESTABLISHED_LEN] = slice(&self.raw, ESTABLISHED_OFF);
        established::Established::parse(&established)
    }

    #[must_use]
    pub const fn timings(&self) -> std1::Std1 {
        let std1: [u8; STANDARD_LEN] = slice(&self.raw, STANDARD_OFF);
        std1::Std1::parse(&std1)
    }

    #[must_use]
    pub const fn descriptors(&self) -> descriptors::Descriptors {
        let descriptors: [u8; DTD_NUM * DESC_LEN] = slice(&self.raw, DTD_OFF);
        descriptors::Descriptors::parse(&descriptors)
    }

    #[must_use]
    pub const fn footer(&self) -> footer::Footer {
        let footer: [u8; FOOTER_LEN] = slice(&self.raw, FOOTER_OFF);
        footer::Footer::parse(&footer)
    }

    #[must_use]
    pub fn validate(&self, raw: &[u8; BLOCK_LEN]) -> Validation {
        Validation::new()
            .then(self.header().validate())
            .err_if(!checksum_ok(raw), "Invalid base checksum")
    }
}
