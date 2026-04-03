//! Detailed Timing Descriptor (DTD).
//!
//! An 18-byte structure describing a single video timing mode with
//! precise parameters including pixel clock, active pixels, blanking,
//! sync polarities, and physical display size.
//!
//! # DTD Structure (18 bytes)
//!
//! | Bytes | Description |
//! |-------|-------------|
//! | 0–1   | Pixel clock (×10 kHz, little-endian) |
//! | 2–4   | Horizontal active + blanking |
//! | 5–7   | Vertical active + blanking |
//! | 8–11  | Front porch, sync width (h/v) |
//! | 12–13 | Image size (mm, little-endian) |
//! | 14    | Border size |
//! | 15–17 | Flags (interlace, stereo, sync type) |

use crate::bit::{get_bits, is_set, pack_bits};
use crate::common::{DESC_LEN, FailureKind, Validation};

const CLK_UNIT: u32 = 10_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stereo {
    None,
    FieldSeqRight,
    FieldSeqLeft,
    TwoWayRightEven,
    TwoWayLeftEven,
    FourWay,
    SideBySide,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Sync {
    Analog {
        bipolar: bool,
        serr: bool,
        rgb: bool,
    },
    DigitalComposite {
        serr: bool,
        h_polar: bool,
    },
    DigitalSeparate {
        v_polar: bool,
        h_polar: bool,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Features {
    interlaced: bool,
    stereo: Stereo,
    sync: Sync,
}

impl Features {
    #[must_use]
    pub const fn interlaced(&self) -> bool {
        self.interlaced
    }
    #[must_use]
    pub const fn stereo(&self) -> Stereo {
        self.stereo
    }
    #[must_use]
    pub const fn sync(&self) -> Sync {
        self.sync
    }
}

/// Timings for a single orientation (horizontal or vertical).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timings {
    active: u16,
    blank: u16,
    front: u16,
    sync: u16,
    size_mm: u16,
    border: u8,
}

impl Timings {
    #[must_use]
    pub const fn active(&self) -> u16 {
        self.active
    }
    #[must_use]
    pub const fn blank(&self) -> u16 {
        self.blank
    }
    #[must_use]
    pub const fn front(&self) -> u16 {
        self.front
    }
    #[must_use]
    pub const fn sync(&self) -> u16 {
        self.sync
    }
    #[must_use]
    pub const fn back(&self) -> u16 {
        self.blank
            .saturating_sub(self.front)
            .saturating_sub(self.sync)
    }
    #[must_use]
    pub const fn total(&self) -> u16 {
        self.active + self.blank
    }
    #[must_use]
    pub const fn size_mm(&self) -> u16 {
        self.size_mm
    }
    #[must_use]
    pub const fn border(&self) -> u8 {
        self.border
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DetailedTiming {
    pixel_clock_10khz: u16,
    horizontal: Timings,
    vertical: Timings,
    features: Features,
}

impl DetailedTiming {
    #[must_use]
    pub const fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        Some(Self {
            pixel_clock_10khz: u16::from_le_bytes([raw[0], raw[1]]),
            horizontal: Timings {
                active: pack_bits(get_bits(raw[4], 4, 7), raw[2], 8),
                blank: pack_bits(get_bits(raw[4], 0, 3), raw[3], 8),
                front: pack_bits(get_bits(raw[11], 6, 7), raw[8], 8),
                sync: pack_bits(get_bits(raw[11], 4, 5), raw[9], 8),
                size_mm: pack_bits(get_bits(raw[14], 4, 7), raw[12], 8),
                border: raw[15],
            },
            vertical: Timings {
                active: pack_bits(get_bits(raw[7], 4, 7), raw[5], 8),
                blank: pack_bits(get_bits(raw[7], 0, 3), raw[6], 8),
                front: pack_bits(get_bits(raw[11], 2, 3), get_bits(raw[10], 4, 7), 4),
                sync: pack_bits(get_bits(raw[11], 0, 1), get_bits(raw[10], 0, 3), 4),
                size_mm: pack_bits(get_bits(raw[14], 0, 3), raw[13], 8),
                border: raw[16],
            },
            features: parse_features(raw[17]),
        })
    }

    #[must_use]
    pub const fn pixel_clock_hz(&self) -> u32 {
        self.pixel_clock_10khz as u32 * CLK_UNIT
    }

    #[must_use]
    pub const fn horizontal(&self) -> &Timings {
        &self.horizontal
    }

    #[must_use]
    pub const fn vertical(&self) -> &Timings {
        &self.vertical
    }

    #[must_use]
    pub fn h_khz(&self) -> f64 {
        f64::from(self.pixel_clock_hz()) / f64::from(self.horizontal.total()) / 1000.0
    }

    #[must_use]
    pub const fn features(&self) -> Features {
        self.features
    }

    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new().fail_if(
            self.pixel_clock_10khz == 0,
            FailureKind::TimingPixelClockIsZero,
        )
    }
}

const fn parse_features(raw: u8) -> Features {
    Features {
        interlaced: is_set(raw, 7),
        stereo: parse_stereo(raw),
        sync: parse_sync(raw),
    }
}

const fn parse_stereo(raw: u8) -> Stereo {
    match ((raw >> 5) & 0b11, raw & 0b0000_0001) {
        (0b00, _) => Stereo::None,
        (0b01, 0) => Stereo::FieldSeqRight,
        (0b10, 0) => Stereo::FieldSeqLeft,
        (0b01, 1) => Stereo::TwoWayRightEven,
        (0b10, 1) => Stereo::TwoWayLeftEven,
        (0b11, 0) => Stereo::FourWay,
        _ => Stereo::SideBySide,
    }
}

const fn parse_sync(raw: u8) -> Sync {
    match (raw >> 3) & 0b0000_0011 {
        0b00 | 0b01 => Sync::Analog {
            bipolar: is_set(raw, 3),
            serr: is_set(raw, 2),
            rgb: is_set(raw, 1),
        },
        0b10 => Sync::DigitalComposite {
            serr: is_set(raw, 2),
            h_polar: is_set(raw, 1),
        },
        _ => Sync::DigitalSeparate {
            v_polar: is_set(raw, 2),
            h_polar: is_set(raw, 1),
        },
    }
}
