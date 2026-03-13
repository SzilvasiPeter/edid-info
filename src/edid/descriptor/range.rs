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

use crate::edid::descriptor::DESC_LEN;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Timing {
    DefaultGtf,
    NoTiming,
    SecondaryGtf(SecondaryGtf),
    Cvt(Cvt),
    Other(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecondaryGtf {
    start_khz: u16,
    c_x2: u8,
    m: u16,
    k: u8,
    j_x2: u8,
}

impl SecondaryGtf {
    #[must_use]
    pub const fn parse(data: [u8; 7]) -> Self {
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
pub enum AspectPref {
    A4_3,
    A16_9,
    A16_10,
    A5_4,
    A15_9,
    Reserved(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools, reason = "Spec-aligned EDID bitfields")]
pub struct Cvt {
    major: u8,
    minor: u8,
    add_clock_0_25_mhz: u8,
    max_active: Option<u16>,
    ar_4_3: bool,
    ar_16_9: bool,
    ar_16_10: bool,
    ar_5_4: bool,
    ar_15_9: bool,
    pref: AspectPref,
    rb: bool,
    std_blank: bool,
    h_shrink: bool,
    h_stretch: bool,
    v_shrink: bool,
    v_stretch: bool,
    pref_v_hz: u8,
}

impl Cvt {
    #[must_use]
    pub fn parse(data: [u8; 7]) -> Self {
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
            0b000 => AspectPref::A4_3,
            0b001 => AspectPref::A16_9,
            0b010 => AspectPref::A16_10,
            0b011 => AspectPref::A5_4,
            0b100 => AspectPref::A15_9,
            v => AspectPref::Reserved(v),
        };
        Self {
            major,
            minor,
            add_clock_0_25_mhz: data[1] >> 2,
            max_active,
            ar_4_3: (data[3] & 0b1000_0000) != 0,
            ar_16_9: (data[3] & 0b0100_0000) != 0,
            ar_16_10: (data[3] & 0b0010_0000) != 0,
            ar_5_4: (data[3] & 0b0001_0000) != 0,
            ar_15_9: (data[3] & 0b0000_1000) != 0,
            pref,
            rb: (data[4] & 0b0001_0000) != 0,
            std_blank: (data[4] & 0b0000_1000) != 0,
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
        self.add_clock_0_25_mhz
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
    pub const fn pref(&self) -> AspectPref {
        self.pref
    }

    #[must_use]
    pub const fn rb(&self) -> bool {
        self.rb
    }

    #[must_use]
    pub const fn std_blank(&self) -> bool {
        self.std_blank
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range {
    v_min_hz: u16,
    v_max_hz: u16,
    h_min_khz: u16,
    h_max_khz: u16,
    pixel_mhz: u16,
    timing: Timing,
    extra: [u8; 7],
}

impl Range {
    #[must_use]
    pub fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        if raw[0] != 0 || raw[1] != 0 || raw[2] != 0 || raw[3] != 0xFD || (raw[4] & 0xF0) != 0 {
            return None;
        }
        let (v_min_hz, v_max_hz) = adjust(raw[5], raw[6], raw[4] & 0b11)?;
        let (h_min_khz, h_max_khz) = adjust(raw[7], raw[8], (raw[4] >> 2) & 0b11)?;
        let mut extra = [0; 7];
        extra.copy_from_slice(&raw[11..DESC_LEN]);
        let timing = match raw[10] {
            0x00 => Timing::DefaultGtf,
            0x01 => Timing::NoTiming,
            0x02 => Timing::SecondaryGtf(SecondaryGtf::parse(extra)),
            0x04 => Timing::Cvt(Cvt::parse(extra)),
            v => Timing::Other(v),
        };
        Some(Self {
            v_min_hz,
            v_max_hz,
            h_min_khz,
            h_max_khz,
            pixel_mhz: u16::from(raw[9]) * 10,
            timing,
            extra,
        })
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
    pub const fn timing(&self) -> Timing {
        self.timing
    }

    #[must_use]
    pub const fn extra(&self) -> &[u8] {
        &self.extra
    }
}

fn adjust(min: u8, max: u8, mode: u8) -> Option<(u16, u16)> {
    match mode {
        0b00 => Some((u16::from(min), u16::from(max))),
        0b10 => Some((u16::from(min), u16::from(max) + 255)),
        0b11 => Some((u16::from(min) + 255, u16::from(max) + 255)),
        _ => None,
    }
}
