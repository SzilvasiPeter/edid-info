//! EDID 1.4 base block (bytes 0–127).
//!
//! The base block is the first 128 bytes of an EDID blob and
//! contains all essential display information including:
//! manufacturer data, display parameters, color characteristics, and timing descriptors.
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
pub mod dmt;
pub mod established;
pub mod footer;
pub mod header;
pub mod std1;

use crate::common::{BLOCK_LEN, FailureKind, Validation, checksum_ok};
use basic::{AnalogType, DisplayType};

/// Base block structure containing header, display parameters, chroma, timings and descriptors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Base<'a> {
    raw: &'a [u8; BLOCK_LEN],
}

impl<'a> Base<'a> {
    /// Creates a new base block from 128 raw bytes.
    #[must_use]
    pub const fn new(raw: &'a [u8; BLOCK_LEN]) -> Self {
        Self { raw }
    }

    /// Returns the header information (manufacturer ID, version, etc.).
    #[must_use]
    pub fn header(&self) -> header::Header {
        header::Header::new(self.raw)
    }

    /// Returns basic display parameters (video input, screen size, etc.).
    #[must_use]
    pub fn basic(&self) -> basic::Basic {
        basic::Basic::new(self.raw)
    }

    /// Returns chromaticity coordinates (color characteristics).
    #[must_use]
    pub fn chroma(&self) -> chroma::Chroma {
        chroma::Chroma::new(self.raw)
    }

    /// Returns the established timing bitmap (common legacy video timings).
    #[must_use]
    pub fn established(&self) -> established::EstablishedLegacy {
        established::EstablishedLegacy::new(self.raw)
    }

    /// Returns the standard timing information.
    #[must_use]
    pub fn timings(&self) -> std1::Std1 {
        std1::Std1::new(self.raw)
    }

    /// Returns the display timing and monitor descriptors.
    #[must_use]
    pub fn descriptors(&self) -> descriptors::Descriptors {
        descriptors::Descriptors::new(self.raw)
    }

    /// Returns the extension flag and checksum.
    #[must_use]
    pub const fn footer(&self) -> footer::Footer {
        footer::Footer::new(self.raw)
    }

    /// Validates the base block.
    #[must_use]
    pub fn validate(&self) -> Validation {
        let chroma_srgb = self.chroma().is_srgb();
        let mono = matches!(
            self.basic().features().display(),
            DisplayType::Analog(AnalogType::MonoGray)
        );
        Validation::new()
            .then(self.header().validate())
            .then(self.basic().validate(chroma_srgb))
            .then(self.chroma().validate(mono))
            .then(self.timings().validate())
            .then(self.descriptors().validate())
            .fail_if(!checksum_ok(self.raw), FailureKind::BaseChecksum)
    }
}
