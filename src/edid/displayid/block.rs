//! `DisplayID` Data Block parsing.
//!
//! Data blocks are the building blocks of `DisplayID`, each containing
//! a specific type of display information.
//!
//! # Data Block Header (3 bytes)
//!
//! | Byte | Description |
//! |------|-------------|
//! | 0    | Block tag |
//! | 1    | Revision (bits 2–0) |
//! | 2    | Payload length |

use crate::edid::displayid::interface::InterfaceFeatures;
use crate::edid::displayid::params::DisplayParams;
use crate::edid::displayid::product::ProductIdent;
use crate::edid::displayid::timing::TimingV7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockTag {
    ProductIdent,
    DisplayParams,
    TimingV7,
    TimingV8,
    TimingV9,
    DynamicRange,
    InterfaceFeatures,
    Stereo,
    TiledTopology,
    ContainerId,
    VendorSpecific,
    CtaDisplayId,
    Reserved(u8),
}

impl BlockTag {
    #[must_use]
    pub const fn parse(raw: u8) -> Self {
        match raw {
            0x20 => Self::ProductIdent,
            0x21 => Self::DisplayParams,
            0x22 => Self::TimingV7,
            0x23 => Self::TimingV8,
            0x24 => Self::TimingV9,
            0x25 => Self::DynamicRange,
            0x26 => Self::InterfaceFeatures,
            0x27 => Self::Stereo,
            0x28 => Self::TiledTopology,
            0x29 => Self::ContainerId,
            0x7E => Self::VendorSpecific,
            0x81 => Self::CtaDisplayId,
            v => Self::Reserved(v),
        }
    }
}

/// A `DisplayID` 2.0 Data Block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataBlock<'a> {
    tag: BlockTag,
    rev: u8,
    data: &'a [u8],
}

impl<'a> DataBlock<'a> {
    #[must_use]
    pub const fn tag(&self) -> BlockTag {
        self.tag
    }

    #[must_use]
    pub const fn rev(&self) -> u8 {
        self.rev
    }

    #[must_use]
    pub const fn data(&self) -> &'a [u8] {
        self.data
    }

    #[must_use]
    pub fn product_ident(&self) -> Option<ProductIdent<'a>> {
        (self.tag == BlockTag::ProductIdent)
            .then(|| ProductIdent::parse(self.data))
            .flatten()
    }

    #[must_use]
    pub fn display_params(&self) -> Option<DisplayParams> {
        (self.tag == BlockTag::DisplayParams)
            .then(|| DisplayParams::parse(self.data))
            .flatten()
    }

    #[must_use]
    pub fn timing_v7(&self) -> Option<TimingV7<'a>> {
        (self.tag == BlockTag::TimingV7)
            .then(|| TimingV7::parse(self.rev, self.data))
            .flatten()
    }

    #[must_use]
    pub fn interface_features(&self) -> Option<InterfaceFeatures<'a>> {
        (self.tag == BlockTag::InterfaceFeatures)
            .then(|| InterfaceFeatures::parse(self.data))
            .flatten()
    }
}

/// Iterator over `DisplayID` data blocks.
pub struct DataBlockIter<'a> {
    pub(crate) raw: &'a [u8],
    pub(crate) at: usize,
    pub(crate) end: usize,
}

impl<'a> Iterator for DataBlockIter<'a> {
    type Item = DataBlock<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at + 3 > self.end {
            return None;
        }

        let tag_raw = self.raw[self.at];
        let rev = self.raw[self.at + 1] & 0x07;
        let len = self.raw[self.at + 2] as usize;
        let start = self.at + 3;
        let end = start + len;

        if end > self.end {
            self.at = self.end;
            return None;
        }

        self.at = end;
        Some(DataBlock {
            tag: BlockTag::parse(tag_raw),
            rev,
            data: &self.raw[start..end],
        })
    }
}
