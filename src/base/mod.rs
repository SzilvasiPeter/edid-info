//! EDID 1.4 base block (bytes 0–127).
//!
//! The base block is the first 128 bytes of an EDID blob and contains
//! all essential display information including manufacturer data, display
//! parameters, color characteristics, and timing descriptors.
//!
//! # Structure
//!
//! | Offset | Size | Description |
//! |--------|------|-------------|
//! | 0–19   | 20   | Header information |
//! | 20–24  | 5    | Basic display parameters |
//! | 25–34  | 10   | Chromaticity coordinates |
//! | 35–37  | 3    | Established timing bitmap |
//! | 38–53  | 16   | Standard timing information |
//! | 54–125 | 72   | Display descriptors |
//! | 126    | 1    | Extension block count |
//! | 127    | 1    | Checksum |
//!
//! # References
//! - [Wikipedia: EDID 1.4 Structure](https://en.wikipedia.org/wiki/Extended_Display_Identification_Data#Structure,_version_1.4)

pub mod basic;
pub mod chroma;
pub mod descriptor;
pub mod descriptors;
pub mod established;
pub mod footer;
pub mod header;
pub mod std1;

use crate::common::{BLOCK_LEN, DESC_LEN, FailureKind, Validation, checksum_ok, slice};
use basic::{BASIC_LEN, BASIC_OFF};
use chroma::{CHROMA_LEN, CHROMA_OFF};
use descriptors::{DTD_NUM, DTD_OFF};
use established::{ESTABLISHED_LEN, ESTABLISHED_OFF};
use footer::{FOOTER_LEN, FOOTER_OFF};
use header::{HEADER_LEN, HEADER_OFF};
use std1::{STANDARD_LEN, STANDARD_OFF};

/// Base block structure containing header, display parameters, chroma, timings and descriptors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Base {
    raw: [u8; BLOCK_LEN],
}

impl Base {
    /// Creates a new base block from 128 raw bytes.
    #[must_use]
    pub const fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        Self { raw: *raw }
    }

    /// Returns the header information (manufacturer ID, version, etc.).
    #[must_use]
    pub const fn header(&self) -> header::Header {
        let header: [u8; HEADER_LEN] = slice(&self.raw, HEADER_OFF);
        header::Header::new(&header)
    }

    /// Returns basic display parameters (video input, screen size, etc.).
    #[must_use]
    pub const fn basic(&self) -> basic::Basic {
        let basic: [u8; BASIC_LEN] = slice(&self.raw, BASIC_OFF);
        basic::Basic::parse(&basic)
    }

    /// Returns chromaticity coordinates (color characteristics).
    #[must_use]
    pub const fn chroma(&self) -> chroma::Chroma {
        let chroma: [u8; CHROMA_LEN] = slice(&self.raw, CHROMA_OFF);
        chroma::Chroma::parse(&chroma)
    }

    /// Returns the established timing bitmap (common legacy video timings).
    #[must_use]
    pub const fn established(&self) -> established::Established {
        let established: [u8; ESTABLISHED_LEN] = slice(&self.raw, ESTABLISHED_OFF);
        established::Established::new(&established)
    }

    /// Returns the standard timing information.
    #[must_use]
    pub const fn timings(&self) -> std1::Std1 {
        let std1: [u8; STANDARD_LEN] = slice(&self.raw, STANDARD_OFF);
        std1::Std1::parse(&std1)
    }

    /// Returns the display timing and monitor descriptors.
    #[must_use]
    pub const fn descriptors(&self) -> descriptors::Descriptors {
        let descriptors: [u8; DTD_NUM * DESC_LEN] = slice(&self.raw, DTD_OFF);
        descriptors::Descriptors::parse(&descriptors)
    }

    /// Returns the extension flag and checksum.
    #[must_use]
    pub const fn footer(&self) -> footer::Footer {
        let footer: [u8; FOOTER_LEN] = slice(&self.raw, FOOTER_OFF);
        footer::Footer::new(&footer)
    }

    /// Validates the base block, including the header and checksum.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new()
            .then(self.header().validate())
            .then(self.basic().validate())
            .then(self.chroma().validate())
            .then(self.timings().validate())
            .then(self.descriptors().validate())
            .fail_if(!checksum_ok(&self.raw), FailureKind::BaseChecksum)
    }
}
