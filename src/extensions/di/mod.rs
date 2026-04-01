//! Display Information Extension (DI-EXT) Block parsing.
//!
//! DI-EXT provides additional display characteristics including gamma,
//! color primaries, and other display-specific parameters.
//!
//! # DI-EXT Block Structure (128 bytes)
//!
//! | Offset | Size | Description |
//! |--------|------|-------------|
//! | 0–1    | 2    | General Information |
//! | 2–13   | 12   | Digital Interface |
//! | 14–19  | 6    | Display Device |
//! | 20–54  | 35   | Display Capabilities & Feature Support Set |
//! | 55–71  | 17   | Unused Bytes (Reserved) |
//! | 72–80  | 9    | Audio Support (Reserved) |
//! | 81–126 | 46   | Display Transfer Characteristic – Gamma |
//! | 127    | 1    | Miscellaneous Items |
//!
//! # References
//!
//! - [VESA Display Information Extension Block Standard](https://glenwing.github.io/docs/VESA-EEDID-DI-EXT-A.pdf)

use crate::common::{BLOCK_LEN, Validation};

/// Display Information Extension (DI-EXT) Block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiExt<'a> {
    raw: &'a [u8; BLOCK_LEN],
}

impl<'a> DiExt<'a> {
    #[must_use]
    pub(crate) const fn parse(raw: &'a [u8; BLOCK_LEN]) -> Self {
        Self { raw }
    }

    /// Returns the raw bytes of the DI-EXT block.
    #[must_use]
    pub const fn raw(&self) -> &[u8; BLOCK_LEN] {
        self.raw
    }

    /// Validates the DI-EXT block.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new()
    }
}
