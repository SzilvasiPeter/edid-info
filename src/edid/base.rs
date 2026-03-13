//! EDID 1.4 base block structure.
//!
//! The base block is the first 128 bytes of an EDID blob and contains
//! all essential display information including manufacturer data, display
//! parameters, color characteristics, and timing descriptors.

use crate::edid::BLOCK_LEN;
use crate::edid::basic::{BASIC_LEN, BASIC_OFF, Basic};
use crate::edid::chroma::{CHROMA_LEN, CHROMA_OFF, Chroma};
use crate::edid::descriptor::DESC_LEN;
use crate::edid::dtd::{DTD_NUM, DTD_OFF, Descriptors};
use crate::edid::established::{ESTABLISHED_LEN, ESTABLISHED_OFF, Established};
use crate::edid::footer::{FOOTER_LEN, FOOTER_OFF, Footer};
use crate::edid::header::{HEADER_LEN, HEADER_OFF, Header};
use crate::edid::std1::{STANDARD_LEN, STANDARD_OFF, Std1};

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
        let header: [u8; HEADER_LEN] = std::array::from_fn(|i| raw[HEADER_OFF + i]);
        let basic: [u8; BASIC_LEN] = std::array::from_fn(|i| raw[BASIC_OFF + i]);
        let chroma: [u8; CHROMA_LEN] = std::array::from_fn(|i| raw[CHROMA_OFF + i]);
        let established: [u8; ESTABLISHED_LEN] = std::array::from_fn(|i| raw[ESTABLISHED_OFF + i]);
        let std1: [u8; STANDARD_LEN] = std::array::from_fn(|i| raw[STANDARD_OFF + i]);
        let dtd: [u8; DTD_NUM * DESC_LEN] = std::array::from_fn(|i| raw[DTD_OFF + i]);
        let footer: [u8; FOOTER_LEN] = std::array::from_fn(|i| raw[FOOTER_OFF + i]);
        Self {
            raw: *raw,
            header: Header::parse(&header),
            basic: Basic::parse(&basic),
            chroma: Chroma::parse(&chroma),
            established: Established::parse(&established),
            timings: Std1::parse(&std1),
            descriptors: Descriptors::parse(&dtd),
            footer: Footer::parse(&footer),
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
