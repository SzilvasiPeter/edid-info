//! EDID parsing for the 128-byte [`base::BaseEdid`] block and [`Extension`]s.
//!
//! Parse flow: validate length, parse base header + checksum, then parse
//! extensions and keep unknown blocks as raw bytes.
//!
//! # Example
//!
//! ```
//! use edid_info::edid::Edid;
//!
//! let raw = include_bytes!("../../tests/data/ACER_EK221Q_H.edid");
//! let edid = Edid::parse(raw).expect("valid EDID");
//!
//! println!("Manufacturer: {:?}", edid.base().header().manufacturer());
//! ```

pub mod base;
pub mod basic;
pub mod bits;
pub mod check;
pub mod chroma;
pub mod cta;
pub mod descriptor;
pub mod dtd;
pub mod established;
pub mod footer;
pub mod header;
pub mod std1;

/// Length of an EDID block (base or extension) in bytes.
pub const BLOCK_LEN: usize = 128;

/// Parsed EDID data with the base block and optional extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid {
    base: base::BaseEdid,
    extensions: Vec<Extension>,
}

/// EDID extension block types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extension {
    /// CEA-861 extension block.
    Cta(cta::Cta),
    /// Unrecognized extension type, stored as raw bytes.
    Unknown([u8; BLOCK_LEN]),
}

impl Edid {
    /// Parses raw EDID bytes if length, header, and checksum are valid.
    #[must_use]
    pub fn parse(raw: &[u8]) -> Option<Self> {
        if raw.len() < BLOCK_LEN || !raw.len().is_multiple_of(BLOCK_LEN) {
            return None;
        }

        let base_raw: [u8; BLOCK_LEN] = std::array::from_fn(|i| raw[i]);
        let base = base::BaseEdid::parse(&base_raw);
        if base.header().pattern() != [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
            || !check::checksum_ok(&base_raw)
        {
            return None;
        }

        let ext_num = base.footer().extension_num() as usize;
        let blocks = raw.len() / BLOCK_LEN - 1;
        if ext_num > blocks {
            return None;
        }

        let extensions = raw[BLOCK_LEN..]
            .chunks_exact(BLOCK_LEN)
            .take(ext_num)
            .map(|chunk| {
                let block: [u8; BLOCK_LEN] = std::array::from_fn(|i| chunk[i]);
                cta::Cta::parse(&block).map_or(Extension::Unknown(block), Extension::Cta)
            })
            .collect();

        Some(Self { base, extensions })
    }

    /// Returns the base block.
    #[must_use]
    pub const fn base(&self) -> &base::BaseEdid {
        &self.base
    }

    /// Returns the extensions.
    #[must_use]
    pub fn extensions(&self) -> &[Extension] {
        &self.extensions
    }
}
