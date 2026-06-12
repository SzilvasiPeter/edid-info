//! Display Operating Range Descriptor.
//!
//! Specifies the supported vertical and horizontal frequency ranges,
//! maximum pixel clock, and timing formula type (GTF, CVT).
//!
//! # Range Descriptor Structure (tag 0xFD)
//!
//! | Byte | Description |
//! |------|-------------|
//! | 5    | Minimum vertical rate (Hz) |
//! | 6    | Maximum vertical rate (Hz) |
//! | 7    | Minimum horizontal rate (kHz) |
//! | 8    | Maximum horizontal rate (kHz) |
//! | 9    | Maximum pixel clock (×10 MHz) |
//! | 10   | Timing formula type |

use crate::common::AspectRatio;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoTimingSupport {
    // TODO: raise validation warning/error when basic display parameters byte 24, bit 0 is *not* set
    DefaultGtf,
    NoInformation,
    SecondaryGtf,
    // TODO: raise validation warning/error when basic display parameters byte 24, bit 0 is *not* set
    Cvt,
    Reserved(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoTimingData {
    None,
    GtfSecondaryCurve(GtfSecondaryCurve),
    CvtSupport(CvtSupport),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GtfSecondaryCurve {
    // TODO: add better name for fields
    start_khz: u16,
    c_x2: u8,
    m: u16,
    k: u8,
    j_x2: u8,
}

impl GtfSecondaryCurve {
    #[must_use]
    pub const fn parse(data: &[u8]) -> Self {
        Self {
            start_khz: u16::from_le_bytes([data[1], 0]) * 2,
            c_x2: data[2],
            m: u16::from_le_bytes([data[3], data[4]]),
            k: data[5],
            j_x2: data[6],
        }
    }

    #[must_use]
    pub const fn start_khz(&self) -> u16 {
        self.start_khz
    }

    #[must_use]
    pub const fn c_x2(&self) -> u8 {
        self.c_x2
    }

    #[must_use]
    pub const fn m(&self) -> u16 {
        self.m
    }

    #[must_use]
    pub const fn k(&self) -> u8 {
        self.k
    }

    #[must_use]
    pub const fn j_x2(&self) -> u8 {
        self.j_x2
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools, reason = "Spec-aligned EDID bitfields")]
pub struct CvtSupport {
    // TODO: add better name for fields
    // TODO: Use the Version from the common module.
    major: u8,
    minor: u8,
    additional_pixel_clock_precision: u8,
    max_active: Option<u16>,
    // TODO: Use list of AspectRatio
    ar_4_3: bool,
    ar_16_9: bool,
    ar_16_10: bool,
    ar_5_4: bool,
    ar_15_9: bool,
    preferred_aspect: Option<AspectRatio>,
    // BlankingSupport
    reduced_blanking: bool,
    standard_blanking: bool,
    // TODO: Refactor the bitmaps as ScalingSupport struct
    h_shrink: bool,
    h_stretch: bool,
    v_shrink: bool,
    v_stretch: bool,
    pref_v_hz: u8,
}

impl CvtSupport {
    #[must_use]
    pub fn parse(data: &[u8]) -> Self {
        let top = data[0];
        let major = (top >> 4) & 0x0F;
        let minor = top & 0x0F;
        let msb = data[1] & 0b11;
        let lsb = data[2];
        let max_active = if lsb == 0 {
            None
        } else {
            Some(((u16::from(msb)) << 8) | u16::from(lsb))
        };
        let pref = match (data[4] >> 5) & 0b111 {
            0b000 => Some(AspectRatio::new(4, 3)),
            0b001 => Some(AspectRatio::new(16, 9)),
            0b010 => Some(AspectRatio::new(16, 10)),
            0b011 => Some(AspectRatio::new(5, 4)),
            0b100 => Some(AspectRatio::new(15, 9)),
            _ => None,
        };
        Self {
            major,
            minor,
            additional_pixel_clock_precision: data[1] >> 2,
            max_active,
            ar_4_3: (data[3] & 0b1000_0000) != 0,
            ar_16_9: (data[3] & 0b0100_0000) != 0,
            ar_16_10: (data[3] & 0b0010_0000) != 0,
            ar_5_4: (data[3] & 0b0001_0000) != 0,
            ar_15_9: (data[3] & 0b0000_1000) != 0,
            preferred_aspect: pref,
            reduced_blanking: (data[4] & 0b0001_0000) != 0,
            standard_blanking: (data[4] & 0b0000_1000) != 0,
            h_shrink: (data[5] & 0b1000_0000) != 0,
            h_stretch: (data[5] & 0b0100_0000) != 0,
            v_shrink: (data[5] & 0b0010_0000) != 0,
            v_stretch: (data[5] & 0b0001_0000) != 0,
            pref_v_hz: data[6],
        }
    }

    #[must_use]
    pub const fn major(&self) -> u8 {
        self.major
    }

    #[must_use]
    pub const fn minor(&self) -> u8 {
        self.minor
    }

    #[must_use]
    pub const fn add_clock_0_25_mhz(&self) -> u8 {
        self.additional_pixel_clock_precision
    }

    #[must_use]
    pub const fn max_active(&self) -> Option<u16> {
        self.max_active
    }

    #[must_use]
    pub const fn ar_4_3(&self) -> bool {
        self.ar_4_3
    }

    #[must_use]
    pub const fn ar_16_9(&self) -> bool {
        self.ar_16_9
    }

    #[must_use]
    pub const fn ar_16_10(&self) -> bool {
        self.ar_16_10
    }

    #[must_use]
    pub const fn ar_5_4(&self) -> bool {
        self.ar_5_4
    }

    #[must_use]
    pub const fn ar_15_9(&self) -> bool {
        self.ar_15_9
    }

    #[must_use]
    pub const fn preferred_aspect(&self) -> Option<AspectRatio> {
        self.preferred_aspect
    }

    #[must_use]
    pub const fn rb(&self) -> bool {
        self.reduced_blanking
    }

    #[must_use]
    pub const fn std_blank(&self) -> bool {
        self.standard_blanking
    }

    #[must_use]
    pub const fn h_shrink(&self) -> bool {
        self.h_shrink
    }

    #[must_use]
    pub const fn h_stretch(&self) -> bool {
        self.h_stretch
    }

    #[must_use]
    pub const fn v_shrink(&self) -> bool {
        self.v_shrink
    }

    #[must_use]
    pub const fn v_stretch(&self) -> bool {
        self.v_stretch
    }

    #[must_use]
    pub const fn pref_v_hz(&self) -> u8 {
        self.pref_v_hz
    }
}

// TODO: Store raw bytes for memory efficiency, and for later reserved byte validation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayRangeLimits {
    v_min_hz: u16,
    v_max_hz: u16,
    h_min_khz: u16,
    h_max_khz: u16,
    pixel_mhz: u16,
    timing_support: VideoTimingSupport,
    timing_data: VideoTimingData,
}

impl DisplayRangeLimits {
    #[must_use]
    pub(super) fn parse(offsets: u8, raw: &[u8; 13]) -> Self {
        let (v_min_hz, v_max_hz) = adjust(raw[0], raw[1], offsets & 0b11);
        let (h_min_khz, h_max_khz) = adjust(raw[2], raw[3], (offsets >> 2) & 0b11);
        let pixel_mhz = u16::from(raw[4]) * 10;
        let (timing_support, timing_data) = match raw[5] {
            0x00 => (VideoTimingSupport::DefaultGtf, VideoTimingData::None),
            0x01 => (VideoTimingSupport::NoInformation, VideoTimingData::None),
            0x02 => (
                VideoTimingSupport::SecondaryGtf,
                VideoTimingData::GtfSecondaryCurve(GtfSecondaryCurve::parse(&raw[6..])),
            ),
            0x04 => (
                VideoTimingSupport::Cvt,
                VideoTimingData::CvtSupport(CvtSupport::parse(&raw[6..])),
            ),
            v => (VideoTimingSupport::Reserved(v), VideoTimingData::None),
        };
        Self {
            v_min_hz,
            v_max_hz,
            h_min_khz,
            h_max_khz,
            pixel_mhz,
            timing_support,
            timing_data,
        }
    }

    #[must_use]
    pub const fn v_min_hz(&self) -> u16 {
        self.v_min_hz
    }

    #[must_use]
    pub const fn v_max_hz(&self) -> u16 {
        self.v_max_hz
    }

    #[must_use]
    pub const fn h_min_khz(&self) -> u16 {
        self.h_min_khz
    }

    #[must_use]
    pub const fn h_max_khz(&self) -> u16 {
        self.h_max_khz
    }

    #[must_use]
    pub const fn pixel_mhz(&self) -> u16 {
        self.pixel_mhz
    }

    #[must_use]
    pub const fn timing(&self) -> VideoTimingSupport {
        self.timing_support
    }

    #[must_use]
    pub const fn timing_data(&self) -> VideoTimingData {
        self.timing_data
    }
}

fn adjust(min: u8, max: u8, mode: u8) -> (u16, u16) {
    match mode {
        0b10 => (u16::from(min), u16::from(max) + 255),
        0b11 => (u16::from(min) + 255, u16::from(max) + 255),
        _ => (u16::from(min), u16::from(max)),
    }
}
