//! EDID 1.4 base block and extension parsing.

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

/// Length of a detailed timing or monitor descriptor in bytes.
pub const DESC_LEN: usize = 18;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edid {
    base: base::BaseEdid,
    extensions: Vec<Extension>,
}

/// EDID Extensions assigned by VESA
///
/// - Timing Extension (00)
/// - Additional Timing Data Block (CTA EDID Timing Extension) (02)
/// - Video Timing Block Extension (VTB-EXT) (10)
/// - EDID 2.0 Extension (20)
/// - Display Information Extension (DI-EXT) (40)
/// - Localized String Extension (LS-EXT) (50)
/// - Microdisplay Interface Extension (MI-EXT) (60)
/// - Display ID Extension (70)
/// - Display Transfer Characteristics Data Block (DTCDB) (A7, AF, BF)
/// - Block Map (F0)
/// - Display Device Data Block (DDDB) (FF): contains information such as subpixel layout
/// - Extension defined by monitor manufacturer (FF): According to LS-EXT, actual contents
///   varies from manufacturer. However, the value is later used by DDDB.
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
        if base.header().magic() != [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
            || !check::checksum_ok(base_raw)
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
            .filter_map(|chunk| {
                let block: &[u8; BLOCK_LEN] = chunk.try_into().ok()?;
                Some(cta::Cta::parse(block).map_or(Extension::Unknown(*block), Extension::Cta))
            })
            .collect();

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
