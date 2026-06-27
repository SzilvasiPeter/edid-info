//! EDID 1.4 base block (bytes 0–127).
//!
//! The base block is the first 128 bytes of an EDID blob and
//! contains all essential display information including:
//! manufacturer data, display parameters, color characteristics, and timing descriptors.
//!
//! # Examples
//!
//! ```rust
//! use edid_info::base::Base;
//! use edid_info::common::{BLOCK_LEN, Version};
//!
//! let mut raw = [0u8; BLOCK_LEN];
//!
//! // Header (bytes 0–19)
//! raw[0..8].copy_from_slice(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]);
//! raw[8..10].copy_from_slice(&[0x04, 0x21]); // Manufacturer: "AAA"
//! raw[10..12].copy_from_slice(&[0x01, 0x00]); // Product code: 1
//! raw[12..16].copy_from_slice(&[0x01, 0x00, 0x00, 0x00]); // Serial: 1
//! raw[16] = 12; // Manufacture week
//! raw[17] = 34; // Manufacture year (1990 + 34 = 2024)
//! raw[18] = 1;  // Major version
//! raw[19] = 4;  // Minor version
//!
//! // Basic display parameters (bytes 20–24)
//! raw[20] = 0xA5; // Digital, 8-bit, DisplayPort
//! raw[21] = 48;   // Width: 48 cm
//! raw[22] = 27;   // Height: 27 cm
//! raw[23] = 120;  // Gamma: 2.20
//! raw[24] = 0x04; // Features: sRGB
//!
//! // Chromaticity (bytes 25–34) — sRGB coordinates
//! raw[25] = 0xEE;
//! raw[26] = 0x91;
//! raw[27] = 163;
//! raw[28] = 84;
//! raw[29] = 76;
//! raw[30] = 153;
//! raw[31] = 38;
//! raw[32] = 15;
//! raw[33] = 80;
//! raw[34] = 84;
//!
//! // Standard timings (bytes 38–53) — all unused
//! raw[38..54].fill(0x01);
//!
//! // First descriptor (bytes 54–71): detailed timing
//! raw[54] = 0x01;
//! raw[55] = 0x01;
//!
//! // Second descriptor (bytes 72–89): product name
//! raw[72] = 0x00;
//! raw[73] = 0x00;
//! raw[75] = 0xFC;
//! raw[77..90].copy_from_slice(b"My Monitor\n\0\0");
//!
//! // Third descriptor (bytes 90–107): dummy
//! raw[90] = 0x00;
//! raw[91] = 0x00;
//! raw[93] = 0x10;
//!
//! // Fourth descriptor (bytes 108–125): dummy
//! raw[108] = 0x00;
//! raw[109] = 0x00;
//! raw[111] = 0x10;
//!
//! // Checksum: set byte 127 so all 128 bytes sum to zero
//! let mut checksum: u8 = 0;
//! let mut i = 0;
//! while i < 127 {
//!     checksum = checksum.wrapping_add(raw[i]);
//!     i += 1;
//! }
//! raw[127] = checksum.wrapping_neg();
//!
//! let base = Base::new(&raw);
//! assert_eq!(base.header().version(), Version { major: 1, minor: 4 });
//! assert!(base.chroma().is_srgb());
//! assert_eq!(base.standard_timings().iter().count(), 0);
//! assert!(base.descriptors().iter().any(|d| matches!(d, edid_info::base::descriptors::Descriptor::Timing(_))));
//! assert_eq!(base.footer().extension_count(), 0);
//! assert!(base.validate().is_valid());
//! ```
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
pub mod standard;

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
    pub const fn established_timings(&self) -> established::EstTimings {
        established::EstTimings::new(self.raw)
    }

    /// Returns the standard timing information.
    #[must_use]
    pub fn standard_timings(&self) -> standard::StdTimings {
        standard::StdTimings::new(self.raw)
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
        let continous_frequency = self.basic().features().continous_frequency();
        let mono = matches!(
            self.basic().features().display(),
            DisplayType::Analog(AnalogType::MonoGray)
        );
        Validation::new()
            .then(self.header().validate())
            .then(self.basic().validate(chroma_srgb))
            .then(self.chroma().validate(mono))
            .then(self.standard_timings().validate())
            .then(self.descriptors().validate(continous_frequency))
            .fail_if(!checksum_ok(self.raw), FailureKind::BaseChecksumMismatch)
    }
}
