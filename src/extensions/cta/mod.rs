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
//! - [Wikipedia: CTA Extension Block](https://en.wikipedia.org/wiki/Extended_Display_Identification_Data#CTA_EDID_Timing_Extension_Block)
//! - [CTA-861-G Standard](https://ia800707.us.archive.org/27/items/CTA-861-G/CTA-861-G.pdf)

pub mod audio;
pub mod block;
pub mod room;
pub mod speaker;
pub mod vendor;
pub mod vic;
pub mod video;

use crate::base::descriptor::dtd::DetailedTiming;
use crate::bit::is_set;
use crate::common::{BLOCK_LEN, DESC_LEN, FailureKind, Validation, checksum_ok};

use block::DataBlockIter;

/// CTA Extension Block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cta<'a> {
    raw: &'a [u8; BLOCK_LEN],
}

impl<'a> Cta<'a> {
    #[must_use]
    pub(crate) const fn parse(raw: &'a [u8; BLOCK_LEN]) -> Self {
        Self { raw }
    }

    #[must_use]
    pub const fn raw(&self) -> &[u8; BLOCK_LEN] {
        self.raw
    }

    #[must_use]
    pub const fn revision(&self) -> u8 {
        self.raw[1]
    }

    #[must_use]
    pub const fn dtd_offset(&self) -> u8 {
        self.raw[2]
    }

    #[must_use]
    pub const fn underscan(&self) -> bool {
        is_set(self.raw[3], 7)
    }

    #[must_use]
    pub const fn basic_audio(&self) -> bool {
        is_set(self.raw[3], 6)
    }

    #[must_use]
    pub const fn ycbcr_444(&self) -> bool {
        is_set(self.raw[3], 5)
    }

    #[must_use]
    pub const fn ycbcr_422(&self) -> bool {
        is_set(self.raw[3], 4)
    }

    #[must_use]
    pub const fn native_dtd_num(&self) -> u8 {
        self.raw[3] & 0b0000_1111
    }

    #[must_use]
    pub const fn data_blocks(&self) -> DataBlockIter<'_> {
        DataBlockIter {
            raw: self.raw,
            at: 4,
            end: self.data_block_end(),
        }
    }

    #[must_use]
    pub const fn checksum(&self) -> u8 {
        self.raw[BLOCK_LEN - 1]
    }

    #[must_use]
    pub fn dtd(&self, i: usize) -> Option<DetailedTiming> {
        let start = self.dtd_start()?;
        let off = start + i * DESC_LEN;
        let end = off + DESC_LEN;
        if end > BLOCK_LEN {
            return None;
        }
        if self.raw[off] == 0 && self.raw[off + 1] == 0 {
            return None;
        }
        let mut raw = [0; DESC_LEN];
        raw.copy_from_slice(&self.raw[off..end]);
        DetailedTiming::parse(&raw)
    }

    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new().fail_if(!checksum_ok(self.raw), FailureKind::CtaChecksum)
    }

    /// Returns the end of the Data Block Collection (DBC).
    ///
    /// Per CTA-861 (v3+):
    /// - `dtd_off` is `0x00`: no DTDs, DBC spans bytes 4–126.
    /// - `dtd_off` is `0x04`: DBC is zero-length (no data blocks).
    /// - Otherwise: DBC spans bytes 4..`dtd_off`.
    const fn data_block_end(self) -> usize {
        let dtd_off = self.raw[2] as usize;
        if dtd_off == 0 || dtd_off > 127 {
            127
        } else if dtd_off >= 4 {
            dtd_off
        } else {
            4
        }
    }

    const fn dtd_start(self) -> Option<usize> {
        let dtd_off = self.raw[2] as usize;
        let last_start = BLOCK_LEN - DESC_LEN;
        if dtd_off >= 4 && dtd_off <= last_start {
            Some(dtd_off)
        } else {
            None
        }
    }
}
