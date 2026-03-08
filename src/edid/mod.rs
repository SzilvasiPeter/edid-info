pub mod base;
pub mod basic;
pub mod bits;
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
                .map_or_else(|| Extension::Unknown(*ext_raw), Extension::Cta);
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
