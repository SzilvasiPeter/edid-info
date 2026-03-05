use crate::edid::descriptor::timing::{DETAILED_LEN, DetailedTiming};

pub const CTA_LEN: usize = 128;
pub const CTA_TAG: u8 = 0b0000_0010;

// CTA data block length is a 5-bit field (bits 4..0), so max payload is (2^5 - 1) 31 bytes.
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
    pub const fn len(&self) -> u8 {
        self.len
    }
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data[..usize::from(self.len)]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cta {
    raw: [u8; CTA_LEN],
}

impl Cta {
    #[must_use]
    pub fn parse(raw: &[u8; CTA_LEN]) -> Option<Self> {
        (raw[0] == CTA_TAG).then_some(Self { raw: *raw })
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
    pub fn data_blocks(&self) -> DataBlockIter<'_> {
        DataBlockIter {
            raw: &self.raw,
            at: 4,
            end: self.dbc_end(),
        }
    }

    #[must_use]
    pub fn dtd(&self, i: usize) -> Option<DetailedTiming> {
        let start = self.dtd_start()?;
        let off = start + i * DETAILED_LEN;
        let end = off + DETAILED_LEN;
        if end > 127 || (self.raw[off] == 0 && self.raw[off + 1] == 0) {
            return None;
        }
        let mut raw = [0; DETAILED_LEN];
        raw.copy_from_slice(&self.raw[off..end]);
        DetailedTiming::parse(&raw)
    }

    const fn dbc_end(&self) -> usize {
        let dtd_off = self.raw[2];
        if dtd_off == 0 {
            127
        } else if dtd_off >= 4 {
            dtd_off as usize
        } else {
            4
        }
    }

    const fn dtd_start(&self) -> Option<usize> {
        let dtd_off = self.raw[2];
        if dtd_off >= 4 && dtd_off < 127 {
            Some(dtd_off as usize)
        } else {
            None
        }
    }
}

pub struct DataBlockIter<'a> {
    raw: &'a [u8; CTA_LEN],
    at: usize,
    end: usize,
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
