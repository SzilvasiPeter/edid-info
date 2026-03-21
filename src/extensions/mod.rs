//! EDID extension blocks.

pub mod cta;

use crate::common::BLOCK_LEN;

/// EDID extension block types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extension {
    /// CEA-861 extension block.
    Cta(cta::Cta),
    /// Unrecognized extension type, stored as raw bytes.
    Unknown([u8; BLOCK_LEN]),
}
