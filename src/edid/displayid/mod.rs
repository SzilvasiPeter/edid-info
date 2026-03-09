//! `DisplayID` 2.0 Extension Block parsing.
//!
//! `DisplayID` is a VESA standard for modern display identification,
//! replacing EDID for newer displays. It supports higher resolutions,
//! refresh rates, and advanced features like tiled displays and
//! dynamic range metadata.
//!
//! # `DisplayID` 2.0 Structure (in 128-byte EDID extension)
//!
//! | Offset | Size | Description |
//! |--------|------|-------------|
//! | 0      | 1    | Tag (0x70) |
//! | 1      | 1    | Reserved |
//! | 2      | 1    | Version (0x20 for 2.0) |
//! | 3      | 1    | Section length |
//! | 4      | 1    | Primary use model |
//! | 5      | 1    | Extension count |
//! | 6+     | var  | Data blocks |
//!
//! # Data Block Types
//!
//! | Tag | Name | Description |
//! |-----|------|-------------|
//! | 0x20 | Product Identification | Vendor, product, serial |
//! | 0x21 | Display Parameters | Size, resolution, color |
//! | 0x22–0x24 | Timing | Video timing descriptors |
//! | 0x25 | Dynamic Range | HDR metadata |
//! | 0x26 | Interface Features | Link capabilities |
//! | 0x28 | Tiled Topology | Multi-panel displays |
//! | 0x29 | Container ID | Unique display identifier |
//!
//! # References
//! - [Wikipedia: Display](https://en.wikipedia.org/wiki/DisplayID#DisplayID_2.0_structures)

use crate::edid::BLOCK_LEN;
use crate::edid::displayid::block::DataBlockIter;

pub mod block;
pub mod interface;
pub mod params;
pub mod product;
pub mod timing;

pub const DISPLAYID_TAG: u8 = 0x70;

/// `DisplayID` 2.0 Extension Block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayId {
    raw: [u8; BLOCK_LEN],
}

impl DisplayId {
    #[must_use]
    pub const fn parse(raw: &[u8; BLOCK_LEN]) -> Option<Self> {
        // EDID Extension tag for DisplayID is 0x70
        if raw[0] != DISPLAYID_TAG {
            return None;
        }
        // DisplayID version (byte 2 of extension block, byte 0 of DisplayID structure)
        // We'll be more lenient for now to see what's in there.
        let version = raw[2];
        if version == 0 {
            return None;
        }
        Some(Self { raw: *raw })
    }

    #[must_use]
    pub const fn version(&self) -> u8 {
        self.raw[2]
    }

    #[must_use]
    pub const fn section_len(&self) -> u8 {
        self.raw[3]
    }

    #[must_use]
    pub const fn primary_use(&self) -> u8 {
        self.raw[4] & 0x0F
    }

    #[must_use]
    pub const fn extension_count(&self) -> u8 {
        self.raw[5]
    }

    #[must_use]
    pub fn data_blocks(&self) -> DataBlockIter<'_> {
        let len = self.section_len() as usize;
        // Data blocks start at index 6 in the 128-byte EDID block (index 4 in DisplayID structure).
        // The section length includes data blocks and the DisplayID checksum.
        let end = 6 + len.min(121); // 127 (checksum) - 6 (header) = 121 max payload
        DataBlockIter {
            raw: &self.raw,
            at: 6,
            end,
        }
    }

    #[must_use]
    pub fn checksum_ok(&self) -> bool {
        crate::edid::check(&self.raw)
    }
}
