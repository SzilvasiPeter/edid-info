//! VESA DMT timing metadata.

use crate::common::{SyncPolarity, Timing};

/// One DMT timing entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dmt {
    id: u8,
    std_code: Option<u16>,
    cvt_code: Option<u32>,
    reduced_blanking: bool,
    pixclk_khz: u32,
    interlaced: bool,
    horizontal: Timing,
    vertical: Timing,
    sync: SyncPolarity,
}

impl Dmt {
    /// Creates a DMT timing entry.
    #[must_use]
    pub const fn new(
        id: u8,
        reduced_blanking: bool,
        pixclk_khz: u32,
        interlaced: bool,
        horizontal: Timing,
        vertical: Timing,
        sync: SyncPolarity,
    ) -> Self {
        Self {
            id,
            std_code: None,
            cvt_code: None,
            reduced_blanking,
            pixclk_khz,
            interlaced,
            horizontal,
            vertical,
            sync,
        }
    }

    #[must_use]
    pub const fn with_std_code(mut self, code: u16) -> Self {
        self.std_code = Some(code);
        self
    }

    #[must_use]
    pub const fn with_cvt_code(mut self, code: u32) -> Self {
        self.cvt_code = Some(code);
        self
    }

    #[must_use]
    pub const fn id(&self) -> u8 {
        self.id
    }

    #[must_use]
    pub const fn std_code(&self) -> Option<u16> {
        self.std_code
    }

    #[must_use]
    pub const fn cvt_code(&self) -> Option<u32> {
        self.cvt_code
    }

    #[must_use]
    pub const fn reduced_blanking(&self) -> bool {
        self.reduced_blanking
    }

    #[must_use]
    pub const fn pixclk_khz(&self) -> u32 {
        self.pixclk_khz
    }

    #[must_use]
    pub const fn interlaced(&self) -> bool {
        self.interlaced
    }

    #[must_use]
    pub const fn horizontal(&self) -> Timing {
        self.horizontal
    }

    #[must_use]
    pub const fn vertical(&self) -> Timing {
        self.vertical
    }

    #[must_use]
    pub const fn sync(&self) -> SyncPolarity {
        self.sync
    }

    #[must_use]
    pub const fn h_freq_hz(&self) -> u32 {
        let p = self.pixclk_khz;
        let h = self.horizontal.total() as u32;
        (p / h) * 1000 + ((p % h) * 1000 + h / 2) / h
    }

    #[must_use]
    pub const fn v_freq_mhz(&self) -> u32 {
        let mut hf = self.h_freq_hz();
        if self.interlaced {
            hf *= 2;
        }
        let v = self.vertical.total() as u32;
        (hf / v) * 1000 + ((hf % v) * 1000 + v / 2) / v
    }
}
