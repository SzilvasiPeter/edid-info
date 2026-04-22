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
use crate::common::{DESC_LEN, FailureKind, Size, Validation, WarningKind};

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

// TODO: Add documentations for the enum and its fields
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Sync {
    Analog {
        bipolar: bool,
        serrations: bool,
        rgb: bool,
    },
    DigitalComposite {
        serrations: bool,
        h_positive: bool,
    },
    DigitalSeparate {
        v_positive: bool,
        h_positive: bool,
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

/// Scan timing parameters for one axis.
///
/// A **line** (horizontal axis) is one row of pixels scanned left to right.
/// A **frame** (vertical axis) is one full screen of lines scanned top to bottom.
/// The GPU and monitor use these values to coordinate when pixels are sent
/// and when to pause for synchronization.
///
/// # Scan Cycle
///
/// ```text
/// ------------------------------------------------------
/// |  visible   | front porch | sync pulse | back porch |
/// | (user sees)|  (settle)   | (trigger)  | (stabilize)|
/// |---active---|--------- blanking interval -----------|
/// ```
///
/// The blanking interval exists because the GPU and monitor need time to
/// synchronize between passes:
///
/// - **Front porch**: the receiver settles after active data ends,
///   ensuring the sync pulse is not mistaken for noise.
/// - **Sync pulse**: the retrace command — both sides reset their position
///   to the start of the next line or frame.
/// - **Back porch**: the sender and receiver stabilize after retrace
///   before visible data begins.
///
/// # Example: 1920×1080 @ 60 Hz (pixel clock 148.50 MHz)
///
/// Horizontal (per line): 1920 active pixels, 280 pixel-times of blanking.
/// The GPU renders 1920 pixels, then pauses for 280 pixel-times before the next line.
///
/// Vertical (per frame): 1080 active lines, 45 line-times of blanking.
/// After 1080 lines, the GPU pauses for 45 line-times before the next frame.
///
/// The refresh rate emerges from the pixel clock and total scan periods:
///
/// ```text
/// Hz = pixel_clock / (horizontal.total() × vertical.total())
///    = 148_500_000 / (2200 × 1125)
///    = 60.00 Hz
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timing {
    active: u16,
    blank: u16,
    front: u16,
    sync: u16,
    border: u8,
}

impl Timing {
    /// Rendered horizontal or vertical pixels.
    #[must_use]
    pub const fn active(&self) -> u16 {
        self.active
    }

    /// Pause duration in pixel-times (line) or line-times (frame).
    #[must_use]
    pub const fn blank(&self) -> u16 {
        self.blank
    }

    /// Front porch duration.
    #[must_use]
    pub const fn front(&self) -> u16 {
        self.front
    }

    /// Sync pulse width.
    #[must_use]
    pub const fn sync(&self) -> u16 {
        self.sync
    }

    /// Back porch duration. This period helps stabilize the signal before the next active pixels are displayed.
    /// Formula: `back_porch = blank_time - (front_porch + sync_pulse)`
    #[must_use]
    pub const fn back(&self) -> u16 {
        self.blank
            .saturating_sub(self.front)
            .saturating_sub(self.sync)
    }

    /// Total scan period: active + blank. Used with the pixel clock to derive the refresh rate.
    #[must_use]
    pub const fn total(&self) -> u16 {
        self.active + self.blank
    }

    /// Border pixels/lines on one edge. Applied to both sides (left+right for horizontal, top+bottom for vertical),
    /// so the total border is twice this value. Typically zero.
    #[must_use]
    pub const fn border(&self) -> u8 {
        self.border
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DetailedTiming {
    pixel_clock_10khz: u16,
    horizontal: Timing,
    vertical: Timing,
    physical: Size,
    features: Features,
}

impl DetailedTiming {
    #[must_use]
    pub const fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        let width_mm = pack_bits(get_bits(raw[14], 4, 7), raw[12], 8);
        let height_mm = pack_bits(get_bits(raw[14], 0, 3), raw[13], 8);

        Some(Self {
            pixel_clock_10khz: u16::from_le_bytes([raw[0], raw[1]]),
            horizontal: Timing {
                active: pack_bits(get_bits(raw[4], 4, 7), raw[2], 8),
                blank: pack_bits(get_bits(raw[4], 0, 3), raw[3], 8),
                front: pack_bits(get_bits(raw[11], 6, 7), raw[8], 8),
                sync: pack_bits(get_bits(raw[11], 4, 5), raw[9], 8),
                border: raw[15],
            },
            vertical: Timing {
                active: pack_bits(get_bits(raw[7], 4, 7), raw[5], 8),
                blank: pack_bits(get_bits(raw[7], 0, 3), raw[6], 8),
                front: pack_bits(get_bits(raw[11], 2, 3), get_bits(raw[10], 4, 7), 4),
                sync: pack_bits(get_bits(raw[11], 0, 1), get_bits(raw[10], 0, 3), 4),
                border: raw[16],
            },
            physical: Size::new(width_mm, height_mm),
            features: parse_features(raw[17]),
        })
    }

    #[must_use]
    pub const fn pixel_clock_hz(&self) -> u32 {
        self.pixel_clock_10khz as u32 * CLK_UNIT
    }

    #[must_use]
    pub const fn horizontal(&self) -> &Timing {
        &self.horizontal
    }

    #[must_use]
    pub const fn vertical(&self) -> &Timing {
        &self.vertical
    }

    #[must_use]
    pub const fn physical(&self) -> Size {
        self.physical
    }

    // TODO: is this correct? shouldn't we use the `Hz = pixel_clock / (horizontal.total() × vertical.total())` formula?
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
        Validation::new()
            .fail_if(
                self.pixel_clock_10khz == 0,
                FailureKind::TimingPixelClockIsZero,
            )
            .warn_if(
                self.physical.width() == 0 || self.physical.height() == 0,
                WarningKind::BasicImageSizeDubious,
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
        (0b01, 0) => Stereo::FieldSeqRight,
        (0b10, 0) => Stereo::FieldSeqLeft,
        (0b01, 1) => Stereo::TwoWayRightEven,
        (0b10, 1) => Stereo::TwoWayLeftEven,
        (0b11, 0) => Stereo::FourWay,
        (0b11, 1) => Stereo::SideBySide,
        _ => Stereo::None,
    }
}

const fn parse_sync(raw: u8) -> Sync {
    match (raw >> 3) & 0b0000_0011 {
        0b00 | 0b01 => Sync::Analog {
            bipolar: is_set(raw, 3),
            serrations: is_set(raw, 2),
            rgb: is_set(raw, 1),
        },
        0b10 => Sync::DigitalComposite {
            serrations: is_set(raw, 2),
            h_positive: is_set(raw, 1),
        },
        _ => Sync::DigitalSeparate {
            v_positive: is_set(raw, 2),
            h_positive: is_set(raw, 1),
        },
    }
}
