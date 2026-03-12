//! EDID 1.4 base block structure.
//!
//! The base block is the first 128 bytes of an EDID blob and contains
//! all essential display information including manufacturer data, display
//! parameters, color characteristics, and timing descriptors.

use crate::edid::BLOCK_LEN;
use crate::edid::basic::Basic;
use crate::edid::chroma::Chroma;
use crate::edid::dtd::Descriptors;
use crate::edid::established::Established;
use crate::edid::footer::Footer;
use crate::edid::header::Header;
use crate::edid::std1::Std1;

/// # EDID 1.4 Base Block Structure (128 bytes)
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
    raw: [u8; BLOCK_LEN],
    header: Header,
    basic: Basic,
    chroma: Chroma,
    established: Established,
    timings: Std1,
    descriptors: Descriptors,
    footer: Footer,
}

impl BaseEdid {
    #[must_use]
    pub fn parse(raw: &[u8; BLOCK_LEN]) -> Self {
        Self {
            raw: *raw,
            header: Header::parse(raw),
            basic: Basic::parse_base(raw),
            chroma: Chroma::parse_base(raw),
            established: Established::parse_base(raw),
            timings: Std1::parse_base(raw),
            descriptors: Descriptors::parse_base(raw),
            footer: Footer::parse(raw),
        }
    }

    #[must_use]
    pub const fn header(&self) -> Header {
        self.header
    }

    #[must_use]
    pub const fn basic(&self) -> Basic {
        self.basic
    }

    #[must_use]
    pub const fn chroma(&self) -> Chroma {
        self.chroma
    }

    #[must_use]
    pub const fn established(&self) -> Established {
        self.established
    }

    #[must_use]
    pub const fn timings(&self) -> Std1 {
        self.timings
    }

    #[must_use]
    pub const fn descriptors(&self) -> Descriptors {
        self.descriptors
    }

    #[must_use]
    pub const fn footer(&self) -> Footer {
        self.footer
    }
}
