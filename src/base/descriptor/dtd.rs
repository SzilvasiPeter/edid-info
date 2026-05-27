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
use crate::common::{
    DESC_LEN, FailureKind, Polarity, Size, SyncPolarity, Timing, Validation, WarningKind,
};

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
pub enum SyncSignal {
    AnalogComposite {
        bipolar: bool,
        serrations: bool,
        source: AnalogSource,
    },
    DigitalComposite {
        serrations: bool,
        h_polarity: Polarity,
    },
    DigitalSeparate(SyncPolarity),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalogSource {
    GreenOnly,
    Rgb,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DetailedTiming {
    pixel_clock_khz: u32,
    horizontal: Timing,
    vertical: Timing,
    physical: Size,
    interlaced: bool,
    stereo: Stereo,
    signal: SyncSignal,
}

impl DetailedTiming {
    #[must_use]
    pub const fn parse(raw: &[u8; DESC_LEN]) -> Self {
        let h_active = pack_bits(get_bits(raw[4], 4, 7), raw[2], 8);
        let h_blank = pack_bits(get_bits(raw[4], 0, 3), raw[3], 8);
        let h_front = pack_bits(get_bits(raw[11], 6, 7), raw[8], 8);
        let h_sync = pack_bits(get_bits(raw[11], 4, 5), raw[9], 8);
        let h_border = raw[15];

        let v_active = pack_bits(get_bits(raw[7], 4, 7), raw[5], 8);
        let v_blank = pack_bits(get_bits(raw[7], 0, 3), raw[6], 8);
        let v_front = pack_bits(get_bits(raw[11], 2, 3), get_bits(raw[10], 4, 7), 4);
        let v_sync = pack_bits(get_bits(raw[11], 0, 1), get_bits(raw[10], 0, 3), 4);
        let v_border = raw[16];

        let width_mm = pack_bits(get_bits(raw[14], 4, 7), raw[12], 8);
        let height_mm = pack_bits(get_bits(raw[14], 0, 3), raw[13], 8);
        Self {
            pixel_clock_khz: u16::from_le_bytes([raw[0], raw[1]]) as u32 * 10,
            horizontal: Timing::new(h_active, h_blank, h_front, h_sync, h_border),
            vertical: Timing::new(v_active, v_blank, v_front, v_sync, v_border),
            physical: Size::new(width_mm, height_mm),
            interlaced: is_set(raw[17], 7),
            stereo: parse_stereo(raw[17]),
            signal: parse_signal(raw[17]),
        }
    }

    #[must_use]
    pub const fn pixel_clock_khz(&self) -> u32 {
        self.pixel_clock_khz
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

    #[must_use]
    pub const fn interlaced(&self) -> bool {
        self.interlaced
    }

    #[must_use]
    pub const fn stereo(&self) -> Stereo {
        self.stereo
    }

    #[must_use]
    pub const fn signal(&self) -> SyncSignal {
        self.signal
    }

    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new()
            .fail_if(
                self.pixel_clock_khz == 0,
                FailureKind::TimingPixelClockIsZero,
            )
            .warn_if(
                self.physical.width() == 0 || self.physical.height() == 0,
                WarningKind::BasicImageSizeDubious,
            )
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

const fn parse_signal(raw: u8) -> SyncSignal {
    const fn polarity(bit: bool) -> Polarity {
        let pos = Polarity::Positive;
        let neg = Polarity::Negative;
        if bit { pos } else { neg }
    }

    let b1 = is_set(raw, 1);
    let b2 = is_set(raw, 2);
    let rgb = AnalogSource::Rgb;
    let green = AnalogSource::GreenOnly;
    match (raw >> 3) & 0b11 {
        0b00 | 0b01 => SyncSignal::AnalogComposite {
            bipolar: is_set(raw, 3),
            serrations: b2,
            source: if b1 { rgb } else { green },
        },
        0b10 => SyncSignal::DigitalComposite {
            serrations: b2,
            h_polarity: polarity(b1),
        },
        _ => SyncSignal::DigitalSeparate(SyncPolarity {
            horizontal: polarity(b1),
            vertical: polarity(b2),
        }),
    }
}
