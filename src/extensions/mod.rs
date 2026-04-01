//! EDID Extensions assigned by VESA
//!
//! - Timing Extension (`00`)
//! - Additional Timing Data Block ([CTA EDID Timing Extension](crate::extensions::cta)) (`02`)
//! - Video Timing Block Extension (VTB-EXT) (`10`)
//! - EDID 2.0 Extension (`20`)
//! - Display Information Extension (DI-EXT) (`40`)
//! - Localized String Extension (LS-EXT) (`50`)
//! - Microdisplay Interface Extension (MI-EXT) (`60`)
//! - Display ID Extension (`70`)
//! - Display Transfer Characteristics Data Block (DTCDB) (`A7`, `AF`, `BF`)
//! - Block Map (`F0`)
//! - Display Device Data Block (DDDB) (`FF`): contains information such as subpixel layout
//! - Extension defined by monitor manufacturer (`FF`): According to LS-EXT, actual contents varies from manufacturer. However, the value is later used by DDDB.

pub mod cta;
pub mod di;

use crate::common::BLOCK_LEN;

/// EDID extension block types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extension<'a> {
    /// Additional Timing Data (CTA) block.
    Cta(cta::Cta<'a>),
    /// Display Information Extension (DI-EXT) block.
    DiExt(di::DiExt<'a>),
    /// Unrecognized extension type, stored as raw bytes.
    Unknown(&'a [u8; BLOCK_LEN]),
}

impl<'a> Extension<'a> {
    /// Parses an extension block from raw bytes.
    ///
    /// Matches on the tag byte to determine the extension type,
    /// then calls the internal parser for that type.
    /// Returns `Unknown` if the tag is not recognized.
    #[must_use]
    pub const fn parse(block: &'a [u8; BLOCK_LEN]) -> Self {
        match block[0] {
            0x02 => Self::Cta(cta::Cta::parse(block)),
            0x40 => Self::DiExt(di::DiExt::parse(block)),
            _ => Self::Unknown(block),
        }
    }
}
