//! EDID 1.4 base block and extension parsing.
//!
//! # EDID 1.4 Base Block Structure (128 bytes)
//!
//! | Offset | Size | Description |
//! |--------|------|-------------|
//! | 0–7    | 8    | Header (magic, manufacturer, product, serial) |
//! | 8–9    | 2    | Manufacturer ID (3-letter code, big-endian) |
//! | 10–11  | 2    | Product code |
//! | 12–15  | 4    | Serial number |
//! | 16     | 1    | Week of manufacture |
//! | 17     | 1    | Year of manufacture (offset from 1990) |
//! | 18     | 1    | EDID version major |
//! | 19     | 1    | EDID version minor |
//! | 20–24  | 5    | Basic display parameters |
//! | 25–34  | 10   | Color characteristics (chromaticity) |
//! | 35–37  | 3    | Established timings |
//! | 38–53  | 16   | Standard timing identification (8 × 2 bytes) |
//! | 54–125 | 72   | Detailed timing descriptors (4 × 18 bytes) |
//! | 126    | 1    | Extension count |
//! | 127    | 1    | Checksum |
//!
//! # References
//!
//! - [Wikipedia: EDID 1.4 Structure](https://en.wikipedia.org/wiki/Extended_Display_Identification_Data#Structure,_version_1.4)

pub mod base;
pub mod basic;
pub mod bits;
pub mod chroma;
pub mod cta;
pub mod descriptor;
pub mod displayid;
pub mod dtd;
pub mod established;
pub mod footer;
pub mod header;
pub mod std1;

/// Length of an EDID block (base or extension) in bytes.
pub const BLOCK_LEN: usize = 128;

/// Length of a detailed timing or monitor descriptor in bytes.
pub const DESC_LEN: usize = 18;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid {
    base: base::BaseEdid,
    extensions: Vec<Extension>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extension {
    Cta(cta::Cta),
    DisplayId(displayid::DisplayId),
    Unknown([u8; BLOCK_LEN]),
}

impl Edid {
    #[must_use]
    pub fn parse(raw: &[u8]) -> Option<Self> {
        if raw.len() < BLOCK_LEN || !raw.len().is_multiple_of(BLOCK_LEN) {
            return None;
        }

        let base_raw: &[u8; BLOCK_LEN] = raw[..BLOCK_LEN].try_into().ok()?;
        let base = base::BaseEdid::parse(base_raw);
        let ext_num = base.footer().extension_num() as usize;

        // Note: we take as many extensions as specified in the base block,
        // or as many as are present in the raw data, whichever is smaller.
        let mut extensions = Vec::with_capacity(ext_num);
        for chunk in raw[BLOCK_LEN..].chunks_exact(BLOCK_LEN).take(ext_num) {
            let ext_raw: &[u8; BLOCK_LEN] = chunk.try_into().ok()?;
            let ext = cta::Cta::parse(ext_raw)
                .map(Extension::Cta)
                .or_else(|| displayid::DisplayId::parse(ext_raw).map(Extension::DisplayId))
                .unwrap_or(Extension::Unknown(*ext_raw));
            extensions.push(ext);
        }

        Some(Self { base, extensions })
    }

    #[must_use]
    pub const fn base(&self) -> &base::BaseEdid {
        &self.base
    }

    #[must_use]
    pub fn extensions(&self) -> &[Extension] {
        &self.extensions
    }
}

#[must_use]
pub fn check(raw: &[u8]) -> bool {
    raw.iter().fold(0u8, |a, b| a.wrapping_add(*b)) == 0
}
