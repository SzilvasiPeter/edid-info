//! CTA-861 Extension Block parsing.
//!
//! The CTA-861 extension is the most common EDID extension, used for
//! HDMI and TV displays. It contains video/audio capabilities, vendor
//! specific data, speaker allocation, and detailed timing descriptors.
//!
//! # CTA-861 Block Structure (128 bytes)
//!
//! | Offset | Size | Description |
//! |--------|------|-------------|
//! | 0      | 1    | Tag (0x02) |
//! | 1      | 1    | Revision |
//! | 2      | 1    | DTD offset |
//! | 3      | 1    | Flags (underscan, audio, YCbCr support) |
//! | 4–N    | var  | Data blocks (video, audio, vendor, speaker) |
//! | N–126  | var  | Detailed timing descriptors |
//! | 127    | 1    | Checksum |
//!
//! # Data Block Types
//!
//! - **Video (tag 2)**: Short Video Descriptors (SVDs) with VIC codes
//! - **Audio (tag 1)**: Short Audio Descriptors (SADs)
//! - **Vendor (tag 3)**: Vendor-specific data (e.g., HDMI VSDB)
//! - **Speaker (tag 4)**: Speaker allocation map
//!
//! # References
//!
//! - [Wikipedia: CTA Extension Block](https://en.wikipedia.org/wiki/Extended_Display_Identification_Data#CTA_EDID_Timing_Extension_Block)
//! - CTA-861-G Specification

pub mod audio;
pub mod block;
pub mod header;
pub mod room;
pub mod speaker;
pub mod vendor;
pub mod vic;
pub mod video;

use crate::edid::descriptor::timing::DetailedTiming;
use crate::edid::{BLOCK_LEN, DESC_LEN, check};

// TODO: Remove the unused imports
pub use audio::{AudioExtFormat, AudioFormat, Sad, SadIter};
pub use block::{BlockTag, DataBlock, DataBlockIter};
pub use header::CTA_TAG;
pub use room::{Coords3, RoomCfg, RoomConfig};
pub use speaker::{Speaker, SpeakerAlloc};
pub use vendor::HdmiVsdb;
pub use vic::Vic;
pub use video::Svd;

/// CTA Extension Block.
/// TODO: Refactor to contain the header information (rev, dtd_num, etc.) explicitely, then remove the `header`
/// TODO: Create "Data Block Collection" field instead of raw bytes
/// TODO: store the checksum here
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cta {
    header: header::Header,
    raw: [u8; BLOCK_LEN],
}

impl Cta {
    #[must_use]
    pub fn parse(raw: &[u8; BLOCK_LEN]) -> Option<Self> {
        if !check::checksum_ok(raw) {
            return None;
        }
        header::Header::parse(raw).map(|header| Self { header, raw: *raw })
    }

    #[must_use]
    pub const fn header(&self) -> header::Header {
        self.header
    }

    #[must_use]
    pub const fn data_blocks(&self) -> DataBlockIter<'_> {
        DataBlockIter {
            raw: &self.raw,
            at: 4,
            end: self.data_block_end(),
        }
    }

    #[must_use]
    pub const fn checksum(&self) -> u8 {
        self.raw[127]
    }

    #[must_use]
    pub fn dtd(&self, i: usize) -> Option<DetailedTiming> {
        let start = self.dtd_start()?;
        let off = start + i * DESC_LEN;
        let end = off + DESC_LEN;
        if end > 127 || (self.raw[off] == 0 && self.raw[off + 1] == 0) {
            return None;
        }
        let mut raw = [0; DESC_LEN];
        raw.copy_from_slice(&self.raw[off..end]);
        DetailedTiming::parse(&raw)
    }

    const fn data_block_end(&self) -> usize {
        let dtd_off = self.header.dtd_off();
        if dtd_off == 0 {
            127
        } else if dtd_off >= 4 {
            dtd_off as usize
        } else {
            4
        }
    }

    const fn dtd_start(&self) -> Option<usize> {
        let dtd_off = self.header.dtd_off();
        if dtd_off >= 4 && dtd_off < 127 {
            Some(dtd_off as usize)
        } else {
            None
        }
    }
}
