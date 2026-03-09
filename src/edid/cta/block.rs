//! CTA Data Block parsing.
//!
//! Data blocks contain collections of descriptors (video, audio, vendor,
//! speaker) and are located at the start of the CTA extension block.
//!
//! # Data Block Header (1 byte)
//!
//! | Bits | Description |
//! |------|-------------|
//! | 7–5  | Block tag (type) |
//! | 4–0  | Payload length (0–31) |

use crate::edid::cta::audio::{Sad, SadIter};
use crate::edid::cta::room::RoomConfig;
use crate::edid::cta::speaker::SpeakerAlloc;
use crate::edid::cta::vendor::HdmiVsdb;
use crate::edid::cta::video::Svd;

/// Maximum length of a CTA data block payload.
///
/// Per CTA-861 spec, the data block collection spans bytes 4 to (`DTD_offset` - 1).
/// The DTD offset minimum is 4, and the maximum practical offset is 127 (before
/// the checksum at byte 127). Thus the maximum data block collection size is
/// 127 - 4 = 123 bytes. However, individual data blocks are limited by their
/// 5-bit length field (max value 31), so no single block can exceed 31 bytes.
const DB_MAX_LEN: usize = 31;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockTag {
    Audio,
    Video,
    Vendor,
    Speaker,
    VesaDtc,
    VideoFmt,
    Extended,
    Reserved(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataBlock {
    tag: BlockTag,
    ext_tag: Option<u8>,
    len: u8,
    data: [u8; DB_MAX_LEN],
}

impl DataBlock {
    #[must_use]
    pub const fn tag(&self) -> BlockTag {
        self.tag
    }
    #[must_use]
    pub const fn ext_tag(&self) -> Option<u8> {
        self.ext_tag
    }
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data[..usize::from(self.len)]
    }

    #[must_use]
    pub fn svd(&self, i: usize) -> Option<Svd> {
        if self.tag != BlockTag::Video {
            return None;
        }
        self.data().get(i).copied().map(Svd::parse)
    }

    fn block_iter<'a, T, F>(&'a self, tag: BlockTag, f: F) -> impl Iterator<Item = T> + 'a
    where
        F: Fn(u8) -> T + 'a,
    {
        let raw = if self.tag == tag { self.data() } else { &[] };
        raw.iter().copied().map(f)
    }

    pub fn svds(&self) -> impl Iterator<Item = Svd> + '_ {
        self.block_iter(BlockTag::Video, Svd::parse)
    }

    #[must_use]
    pub fn sad(&self, i: usize) -> Option<Sad> {
        (self.tag == BlockTag::Audio)
            .then(|| {
                let at = i * 3;
                let raw = self.data();
                (at + 2 < raw.len()).then(|| Sad::parse(raw[at], raw[at + 1], raw[at + 2]))
            })
            .flatten()
    }

    pub fn sads(&self) -> impl Iterator<Item = Sad> + '_ {
        let raw = if self.tag == BlockTag::Audio {
            self.data()
        } else {
            &[]
        };
        SadIter { raw, at: 0 }
    }

    #[must_use]
    pub fn vendor_oui(&self) -> Option<u32> {
        if self.tag != BlockTag::Vendor {
            return None;
        }
        let raw = self.data();
        if raw.len() < 3 {
            return None;
        }
        Some(u32::from(raw[0]) | (u32::from(raw[1]) << 8) | (u32::from(raw[2]) << 16))
    }

    #[must_use]
    pub fn hdmi_vsdb(&self) -> Option<HdmiVsdb> {
        if self.tag != BlockTag::Vendor {
            return None;
        }
        HdmiVsdb::parse(self.data())
    }

    #[must_use]
    pub fn speaker_alloc(&self) -> Option<SpeakerAlloc> {
        if self.tag != BlockTag::Speaker {
            return None;
        }
        let raw = self.data();
        if raw.len() < 3 {
            return None;
        }
        Some(SpeakerAlloc::parse(raw[0], raw[1], raw[2]))
    }

    #[must_use]
    pub fn room_config(&self) -> Option<RoomConfig> {
        if self.tag != BlockTag::Extended || self.ext_tag != Some(13) {
            return None;
        }
        RoomConfig::parse(self.data())
    }
}

pub struct DataBlockIter<'a> {
    pub raw: &'a [u8],
    pub at: usize,
    pub end: usize,
}

impl Iterator for DataBlockIter<'_> {
    type Item = DataBlock;
    fn next(&mut self) -> Option<Self::Item> {
        if self.at >= self.end {
            return None;
        }
        let block_header = self.raw[self.at];
        let tag_raw = block_header >> 5;
        let len = usize::from(block_header & 0b0001_1111);
        let next = self.at + 1 + len;
        if next > self.end {
            self.at = self.end;
            return None;
        }
        let mut data = [0; DB_MAX_LEN];
        data[..len].copy_from_slice(&self.raw[self.at + 1..next]);
        self.at = next;
        let tag = match tag_raw {
            1 => BlockTag::Audio,
            2 => BlockTag::Video,
            3 => BlockTag::Vendor,
            4 => BlockTag::Speaker,
            5 => BlockTag::VesaDtc,
            6 => BlockTag::VideoFmt,
            7 => BlockTag::Extended,
            _ => BlockTag::Reserved(tag_raw),
        };
        let ext_tag = (tag_raw == 7 && len > 0).then_some(data[0]);
        Some(DataBlock {
            tag,
            ext_tag,
            len: block_header & 0b0001_1111,
            data,
        })
    }
}
