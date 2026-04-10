//! Established timing bitmap (bytes 35–37).
//!
//! Supported bitmap for (formerly) very common timing modes.
//!
//! # Structure
//! | Byte | Description |
//! |------|-------------|
//! | 35   | 720×400 - 800×600 displays |
//! | 36   | 800×600 - 1280×1024 displays |
//! | 37   | 1152×870 + 7 manufacturer-specific display modes |

use crate::common::BLOCK_LEN;

/// Established timing offset in the base block.
pub const ESTABLISHED_OFF: usize = 35;

/// Established timing length in bytes.
pub const ESTABLISHED_LEN: usize = 3;

/// Established timing values from bytes 35–37.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstablishedTiming {
    /// 720x400 @ 70 Hz (IBM, VGA).
    T720x400_70,
    /// 720x400 @ 88 Hz (IBM, XGA2).
    T720x400_88,
    /// 640x480 @ 60 Hz (IBM, VGA).
    T640x480_60,
    /// 640x480 @ 67 Hz (Apple, Mac II).
    T640x480_67,
    /// 640x480 @ 72 Hz (VESA).
    T640x480_72,
    /// 640x480 @ 75 Hz (VESA).
    T640x480_75,
    /// 800x600 @ 56 Hz (VESA).
    T800x600_56,
    /// 800x600 @ 60 Hz (VESA).
    T800x600_60,
    /// 800x600 @ 72 Hz (VESA).
    T800x600_72,
    /// 800x600 @ 75 Hz (VESA).
    T800x600_75,
    /// 832x624 @ 75 Hz (Apple, Mac II).
    T832x624_75,
    /// 1024x768 @ 87 Hz interlaced (IBM).
    T1024x768_87I,
    /// 1024x768 @ 60 Hz (VESA).
    T1024x768_60,
    /// 1024x768 @ 70 Hz (VESA).
    T1024x768_70,
    /// 1024x768 @ 75 Hz (VESA).
    T1024x768_75,
    /// 1280x1024 @ 75 Hz (VESA).
    T1280x1024_75,
    /// 1152x870 @ 75 Hz (Apple, Mac II).
    T1152x870_75,
}

/// Established timing bitmap containing supported timings and manufacturer bits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Established {
    supported_timings: [Option<EstablishedTiming>; 17],
    manufacturer_bits: u8,
}

impl Established {
    /// Parses the established timing bitmap from base block bytes.
    ///
    /// | Byte | Bit | Description |
    /// |------|-----|-------------|
    /// | 35   | 7   | 720×400 @ 70 Hz |
    /// |      | 6   | 720×400 @ 88 Hz |
    /// |      | 5   | 640×480 @ 60 Hz |
    /// |      | 4   | 640×480 @ 67 Hz |
    /// |      | 3   | 640×480 @ 72 Hz |
    /// |      | 2   | 640×480 @ 75 Hz |
    /// |      | 1   | 800×600 @ 56 Hz |
    /// |      | 0   | 800×600 @ 60 Hz |
    /// | 36   | 7   | 800×600 @ 72 Hz |
    /// |      | 6   | 800×600 @ 75 Hz |
    /// |      | 5   | 832×624 @ 75 Hz |
    /// |      | 4   | 1024×768i @ 87 Hz |
    /// |      | 3   | 1024×768 @ 60 Hz |
    /// |      | 2   | 1024×768 @ 70 Hz |
    /// |      | 1   | 1024×768 @ 75 Hz |
    /// |      | 0   | 1280×1024 @ 75 Hz |
    /// | 37   | 7   | 1152×870 @ 75 Hz |
    /// |      | 6–0 | Other manufacturer-specific display modes |
    #[must_use]
    pub fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        let established = &raw[ESTABLISHED_OFF..ESTABLISHED_OFF + ESTABLISHED_LEN];
        Self {
            supported_timings: [
                // Established I (Byte 35)
                flag(established[0], 0x80, EstablishedTiming::T720x400_70),
                flag(established[0], 0x40, EstablishedTiming::T720x400_88),
                flag(established[0], 0x20, EstablishedTiming::T640x480_60),
                flag(established[0], 0x10, EstablishedTiming::T640x480_67),
                flag(established[0], 0x08, EstablishedTiming::T640x480_72),
                flag(established[0], 0x04, EstablishedTiming::T640x480_75),
                flag(established[0], 0x02, EstablishedTiming::T800x600_56),
                flag(established[0], 0x01, EstablishedTiming::T800x600_60),
                // Established II (Byte 36)
                flag(established[1], 0x80, EstablishedTiming::T800x600_72),
                flag(established[1], 0x40, EstablishedTiming::T800x600_75),
                flag(established[1], 0x20, EstablishedTiming::T832x624_75),
                flag(established[1], 0x10, EstablishedTiming::T1024x768_87I),
                flag(established[1], 0x08, EstablishedTiming::T1024x768_60),
                flag(established[1], 0x04, EstablishedTiming::T1024x768_70),
                flag(established[1], 0x02, EstablishedTiming::T1024x768_75),
                flag(established[1], 0x01, EstablishedTiming::T1280x1024_75),
                // TODO: This byte is not indicating the established III, the established III is definied under the std3.rs module
                // Established III (Byte 37)
                flag(established[2], 0x80, EstablishedTiming::T1152x870_75),
            ],
            manufacturer_bits: established[2] & 0x7F,
        }
    }

    /// Returns the supported established timings.
    #[must_use]
    pub const fn supported(&self) -> [Option<EstablishedTiming>; 17] {
        self.supported_timings
    }

    /// Returns the reserved manufacturer bits.
    #[must_use]
    pub const fn manufacturer_bits(&self) -> u8 {
        self.manufacturer_bits
    }
}

impl core::fmt::Display for EstablishedTiming {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::T720x400_70 => write!(f, "720x400 @ 70 Hz (IBM, VGA)"),
            Self::T720x400_88 => write!(f, "720x400 @ 88 Hz (IBM, XGA2)"),
            Self::T640x480_60 => write!(f, "640x480 @ 60 Hz (IBM, VGA)"),
            Self::T640x480_67 => write!(f, "640x480 @ 67 Hz (Apple, Mac II)"),
            Self::T640x480_72 => write!(f, "640x480 @ 72 Hz (VESA)"),
            Self::T640x480_75 => write!(f, "640x480 @ 75 Hz (VESA)"),
            Self::T800x600_56 => write!(f, "800x600 @ 56 Hz (VESA)"),
            Self::T800x600_60 => write!(f, "800x600 @ 60 Hz (VESA)"),
            Self::T800x600_72 => write!(f, "800x600 @ 72 Hz (VESA)"),
            Self::T800x600_75 => write!(f, "800x600 @ 75 Hz (VESA)"),
            Self::T832x624_75 => write!(f, "832x624 @ 75 Hz (Apple, Mac II)"),
            Self::T1024x768_87I => write!(f, "1024x768i @ 87 Hz (IBM)"),
            Self::T1024x768_60 => write!(f, "1024x768 @ 60 Hz (VESA)"),
            Self::T1024x768_70 => write!(f, "1024x768 @ 70 Hz (VESA)"),
            Self::T1024x768_75 => write!(f, "1024x768 @ 75 Hz (VESA)"),
            Self::T1280x1024_75 => write!(f, "1280x1024 @ 75 Hz (VESA)"),
            Self::T1152x870_75 => write!(f, "1152x870 @ 75 Hz (Apple, Mac II)"),
        }
    }
}

const fn flag(byte: u8, mask: u8, val: EstablishedTiming) -> Option<EstablishedTiming> {
    if (byte & mask) != 0 { Some(val) } else { None }
}
