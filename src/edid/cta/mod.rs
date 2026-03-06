//! CTA-861 Extension Block parsing.
//!
//! # References
//! - <https://en.wikipedia.org/wiki/Extended_Display_Identification_Data#CTA_Extension_Block>

pub mod audio;
pub mod block;
pub mod header;
pub mod speaker;
pub mod vendor;
pub mod vic;
pub mod video;

use crate::edid::descriptor::timing::{DETAILED_LEN, DetailedTiming};

pub use audio::{AudioExtFormat, AudioFormat, Sad, SadIter};
pub use block::{BlockTag, DataBlock, DataBlockIter};
pub use header::{CTA_LEN, CTA_TAG};
pub use speaker::{Speaker, SpeakerAlloc};
pub use vendor::HdmiVsdb;
pub use vic::Vic;
pub use video::{Svd, SvdIter};

/// CTA Extension Block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cta {
    raw: [u8; CTA_LEN],
}

impl Cta {
    #[must_use]
    pub fn parse(raw: &[u8; CTA_LEN]) -> Option<Self> {
        header::Header::parse(raw).map(|_| Self { raw: *raw })
    }

    #[must_use]
    pub const fn rev(&self) -> u8 {
        self.raw[1]
    }

    #[must_use]
    pub const fn native_dtd_num(&self) -> u8 {
        self.raw[3] & 0b0000_1111
    }

    #[must_use]
    pub const fn underscan(&self) -> bool {
        (self.raw[3] & 0b1000_0000) != 0
    }

    #[must_use]
    pub const fn basic_audio(&self) -> bool {
        (self.raw[3] & 0b0100_0000) != 0
    }

    #[must_use]
    pub const fn ycbcr_444(&self) -> bool {
        (self.raw[3] & 0b0010_0000) != 0
    }

    #[must_use]
    pub const fn ycbcr_422(&self) -> bool {
        (self.raw[3] & 0b0001_0000) != 0
    }

    #[must_use]
    pub const fn checksum(&self) -> u8 {
        self.raw[127]
    }

    #[must_use]
    pub fn checksum_ok(&self) -> bool {
        self.raw.iter().fold(0u8, |sum, b| sum.wrapping_add(*b)) == 0
    }

    #[must_use]
    pub const fn data_blocks(&self) -> DataBlockIter<'_> {
        let dtd_off = self.raw[2];
        let end = if dtd_off == 0 {
            127
        } else if dtd_off >= 4 {
            dtd_off as usize
        } else {
            4
        };
        DataBlockIter {
            raw: &self.raw,
            at: 4,
            end,
        }
    }

    #[must_use]
    pub fn dtd(&self, i: usize) -> Option<DetailedTiming> {
        let dtd_off = self.raw[2];
        let start = if (4..127).contains(&dtd_off) {
            dtd_off as usize
        } else {
            return None;
        };
        let off = start + i * DETAILED_LEN;
        let end = off + DETAILED_LEN;
        if end > 127 || (self.raw[off] == 0 && self.raw[off + 1] == 0) {
            return None;
        }
        let mut raw = [0; DETAILED_LEN];
        raw.copy_from_slice(&self.raw[off..end]);
        DetailedTiming::parse(&raw)
    }
}
