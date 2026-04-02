//! CTA Short Video Descriptors (SVDs).
//!
//! SVDs reference VIC codes to indicate supported video formats.
//! The MSB indicates whether the format is the display's native
//! resolution.
//!
//! # SVD Byte Encoding
//!
//! | Bit | Description |
//! |-----|-------------|
//! | 7   | Native flag (VIC 1-64) or MSB (VIC 193-219) |
//! | 6-0 | VIC code (1-127 for 7-bit, 65-91 for 8-bit) |
//!
//! For VIC 1-64: bit 7 is the native flag (0 for non-native, 1 for native)
//! For VIC 65-127: bit 7 must be 0 (7-bit VIC)
//! For 8-bit VICs: bit 7 is MSB, VIC = 128 + (bits 0-6)
//! Only a subset of 8-bit VICs are defined (e.g. 193-219)

use super::vic::Vic;

/// Short Video Descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Svd {
    vic: u8,
    native: bool,
}

impl Svd {
    #[must_use]
    pub const fn parse(raw: u8) -> Self {
        let vic_code = raw & 0b0111_1111;
        let msb_or_native = (raw & 0b1000_0000) != 0;

        let vic = if msb_or_native && vic_code >= 65 {
            vic_code | 0b1000_0000
        } else {
            vic_code
        };
        let native = msb_or_native && vic_code > 0 && vic_code <= 64;

        Self { vic, native }
    }

    #[must_use]
    pub const fn vic(&self) -> u8 {
        self.vic
    }
    #[must_use]
    pub const fn native(&self) -> bool {
        self.native
    }
    #[must_use]
    pub const fn timing(&self) -> Option<Vic> {
        Vic::from_vic(self.vic)
    }
}
