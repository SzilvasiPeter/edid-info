//! CTA-861 Extension Block parsing.
//!
//! # References
//! - <https://en.wikipedia.org/wiki/Extended_Display_Identification_Data#CTA_Extension_Block>

pub mod audio;
pub mod block;
pub mod header;
pub mod room;
pub mod speaker;
pub mod vendor;
pub mod vic;
pub mod video;

use crate::edid::descriptor::timing::DetailedTiming;
use crate::edid::{BLOCK_LEN, DESC_LEN};

pub use audio::{AudioExtFormat, AudioFormat, Sad, SadIter};
pub use block::{BlockTag, DataBlock, DataBlockIter};
pub use header::CTA_TAG;
pub use room::{Coords3, RoomCfg, RoomConfig};
pub use speaker::{Speaker, SpeakerAlloc};
pub use vendor::HdmiVsdb;
pub use vic::Vic;
pub use video::Svd;

/// CTA Extension Block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cta {
    header: header::Header,
    raw: [u8; BLOCK_LEN],
}

impl Cta {
    #[must_use]
    pub fn parse(raw: &[u8; BLOCK_LEN]) -> Option<Self> {
        header::Header::parse(raw).map(|header| Self { header, raw: *raw })
    }

    #[must_use]
    pub const fn header(&self) -> header::Header {
        self.header
    }

    #[must_use]
    pub const fn rev(&self) -> u8 {
        self.header.rev()
    }

    #[must_use]
    pub const fn native_dtd_num(&self) -> u8 {
        self.header.native_dtd_num()
    }

    #[must_use]
    pub const fn underscan(&self) -> bool {
        self.header.underscan()
    }

    #[must_use]
    pub const fn basic_audio(&self) -> bool {
        self.header.basic_audio()
    }

    #[must_use]
    pub const fn ycbcr_444(&self) -> bool {
        self.header.ycbcr_444()
    }

    #[must_use]
    pub const fn ycbcr_422(&self) -> bool {
        self.header.ycbcr_422()
    }

    #[must_use]
    pub const fn checksum(&self) -> u8 {
        self.raw[127]
    }

    #[must_use]
    pub fn checksum_ok(&self) -> bool {
        crate::edid::check(&self.raw)
    }

    #[must_use]
    pub const fn data_blocks(&self) -> DataBlockIter<'_> {
        DataBlockIter {
            raw: &self.raw,
            at: 4,
            end: self.header.data_block_end(),
        }
    }

    #[must_use]
    pub fn dtd(&self, i: usize) -> Option<DetailedTiming> {
        let start = self.header.dtd_start()?;
        let off = start + i * DESC_LEN;
        let end = off + DESC_LEN;
        if end > 127 || (self.raw[off] == 0 && self.raw[off + 1] == 0) {
            return None;
        }
        let mut raw = [0; DESC_LEN];
        raw.copy_from_slice(&self.raw[off..end]);
        DetailedTiming::parse(&raw)
    }
}
