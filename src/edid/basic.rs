//! Basic display parameters (bytes 20–24).
//!
//! Contains video input definition (analog/digital), display size, gamma, and feature support flags.
//!
//! # Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 20   | Video input definition |
//! | 21   | Horizontal display size (cm) |
//! | 22   | Vertical display size (cm) |
//! | 23   | Gamma (value - 1.0, scaled by 100) |
//! | 24   | Feature support flags |

use crate::edid::bits::{get_bits, is_set};

pub const BASIC_OFF: usize = 20;
pub const BASIC_LEN: usize = 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Basic {
    input: VideoInput,
    width_cm: u8,
    height_cm: u8,
    gamma: u8,
    features: Features,
}

impl Basic {
    #[must_use]
    pub const fn parse(raw: &[u8; BASIC_LEN]) -> Self {
        let input = VideoInput::parse(raw[0]);
        let is_digital = matches!(input.kind(), InputKind::Digital { .. });
        Self {
            input,
            width_cm: raw[1],
            height_cm: raw[2],
            gamma: raw[3],
            features: Features::parse(raw[4], is_digital),
        }
    }

    /// Video input parameters bitmap
    #[must_use]
    pub const fn input(&self) -> VideoInput {
        self.input
    }

    /// Horizontal screen size, in centimetres (range 1–255).
    /// If vertical screen size is 0, landscape aspect ratio (range 1.00–3.54),
    /// datavalue = (AR×100) − 99 (example: 16:9, 79; 4:3, 34.)
    #[must_use]
    pub const fn width_cm(&self) -> u8 {
        self.width_cm
    }

    /// Vertical screen size, in centimetres.
    /// If horizontal screen size is 0, portrait aspect ratio (range 0.28–0.99),
    /// datavalue = (100/AR) − 99 (example: 9:16, 79; 3:4, 34.)
    /// If both bytes are 0, screen size and aspect ratio are undefined (e.g. projector)
    #[must_use]
    pub const fn height_cm(&self) -> u8 {
        self.height_cm
    }

    /// Display gamma, factory default (range 1.00–3.54),
    /// datavalue = (gamma×100) − 100 = (gamma − 1)×100.
    /// If 255, gamma is defined by DI-EXT block.
    #[must_use]
    pub const fn gamma_raw(&self) -> u8 {
        self.gamma
    }

    /// Supported features bitmap
    #[must_use]
    pub const fn features(&self) -> Features {
        self.features
    }
}

/// Video input type (Bit 7 of Byte 20).
///
/// | Value | Description |
/// |-------|-------------|
/// | 1 | Digital input |
/// | 0 | Analog input |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputKind {
    /// Digital input with bit depth and interface.
    Digital { depth: BitDepth, iface: Interface },
    /// Analog input with signal level and sync options.
    Analog {
        level: Level,
        /// Blank-to-black setup (pedestal) expected.
        setup: bool,
        /// Separate sync supported.
        sep: bool,
        /// Composite sync (on `HSync`) supported.
        comp: bool,
        /// Sync on green supported.
        sog: bool,
        /// `VSync` pulse must be serrated when composite or sync-on-green is used.
        serr: bool,
    },
}

/// Bit depth (Bits 6–4 of Byte 20 when Digital).
///
/// | Value | Description |
/// |-------|-------------|
/// | 000 | undefined |
/// | 001 | 6 bits per color |
/// | 010 | 8 bits per color |
/// | 011 | 10 bits per color |
/// | 100 | 12 bits per color |
/// | 101 | 14 bits per color |
/// | 110 | 16 bits per color |
/// | 111 | reserved |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BitDepth {
    Undef,
    B6,
    B8,
    B10,
    B12,
    B14,
    B16,
    Reserved,
}

/// Video interface (Bits 3–0 of Byte 20 when Digital).
///
/// | Value | Description |
/// |-------|-------------|
/// | 0000 | undefined |
/// | 0001 | DVI |
/// | 0010 | HDMIa |
/// | 0011 | HDMIb |
/// | 0100 | MDDI |
/// | 0101 | DisplayPort |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Interface {
    Undef,
    Dvi,
    HdmiA,
    HdmiB,
    Mddi,
    DisplayPort,
    Other(u8),
}

/// Video white and sync levels, relative to blank (Bits 6–5 of Byte 20 when Analog).
///
/// | Value | Voltage Levels |
/// |-------|----------------|
/// | 00 | +0.7/−0.3 V |
/// | 01 | +0.714/−0.286 V |
/// | 10 | +1.0/−0.4 V |
/// | 11 | +0.7/0 V (EVC) |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Level {
    V700_300,
    V714_286,
    V1000_400,
    V700_000,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VideoInput {
    kind: InputKind,
}

impl VideoInput {
    #[must_use]
    pub const fn parse(raw: u8) -> Self {
        let kind = if is_set(raw, 7) {
            let depth = match get_bits(raw, 0b0111_0000, 4) {
                0b000 => BitDepth::Undef,
                0b001 => BitDepth::B6,
                0b010 => BitDepth::B8,
                0b011 => BitDepth::B10,
                0b100 => BitDepth::B12,
                0b101 => BitDepth::B14,
                0b110 => BitDepth::B16,
                _ => BitDepth::Reserved,
            };
            let iface = match get_bits(raw, 0b0000_1111, 0) {
                0 => Interface::Undef,
                1 => Interface::Dvi,
                2 => Interface::HdmiA,
                3 => Interface::HdmiB,
                4 => Interface::Mddi,
                5 => Interface::DisplayPort,
                v => Interface::Other(v),
            };
            InputKind::Digital { depth, iface }
        } else {
            let level = match get_bits(raw, 0b0110_0000, 5) {
                0b00 => Level::V700_300,
                0b01 => Level::V714_286,
                0b10 => Level::V1000_400,
                _ => Level::V700_000,
            };
            InputKind::Analog {
                level,
                setup: is_set(raw, 4),
                sep: is_set(raw, 3),
                comp: is_set(raw, 2),
                sog: is_set(raw, 1),
                serr: is_set(raw, 0),
            }
        };
        Self { kind }
    }

    /// Digital or analog video input type.
    #[must_use]
    pub const fn kind(&self) -> InputKind {
        self.kind
    }
}

/// Display type (analog or digital) for features bitmap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayType {
    Digital(DigitalType),
    Analog(AnalogType),
}

/// Analog display type (Bits 4-3 of Byte 24 when Analog).
///
/// | Value | Description |
/// |-------|-------------|
/// | 00 | monochrome or grayscale |
/// | 01 | RGB color |
/// | 10 | non-RGB color |
/// | 11 | undefined |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalogType {
    MonoGray,
    Rgb,
    NonRgb,
    Undef,
}

/// Digital display type (Bits 4-3 of Byte 24 when Digital).
///
/// | Value | Description |
/// |-------|-------------|
/// | 00 | RGB 4:4:4 |
/// | 01 | RGB 4:4:4 + YCrCb 4:4:4 |
/// | 10 | RGB 4:4:4 + YCrCb 4:2:2 |
/// | 11 | RGB 4:4:4 + YCrCb 4:4:4 + YCrCb 4:2:2 |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DigitalType {
    Rgb444,
    Rgb444Y444,
    Rgb444Y422,
    Rgb444Y444Y422,
}

/// Feature support flags (Byte 24).
///
/// | Bit | Description |
/// |-----|-------------|
/// | 7 | DPMS standby supported |
/// | 6 | DPMS suspend supported |
/// | 5 | DPMS active-off supported |
/// | 4–3 | Display type |
/// | 2 | Standard sRGB colour space |
/// | 1 | Preferred timing mode |
/// | 0 | Continuous timings with GTF or CVT |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[expect(clippy::struct_excessive_bools, reason = "Spec-aligned EDID bitfields")]
pub struct Features {
    standby: bool,
    suspend: bool,
    active_off: bool,
    display: DisplayType,
    srgb: bool,
    pref_timing_mode: bool,
    continuous: bool,
}

impl Features {
    #[must_use]
    pub const fn parse(raw: u8, is_digital: bool) -> Self {
        let display = if is_digital {
            DisplayType::Digital(match get_bits(raw, 0b0001_1000, 3) {
                0b00 => DigitalType::Rgb444,
                0b01 => DigitalType::Rgb444Y444,
                0b10 => DigitalType::Rgb444Y422,
                _ => DigitalType::Rgb444Y444Y422,
            })
        } else {
            DisplayType::Analog(match get_bits(raw, 0b0001_1000, 3) {
                0b00 => AnalogType::MonoGray,
                0b01 => AnalogType::Rgb,
                0b10 => AnalogType::NonRgb,
                _ => AnalogType::Undef,
            })
        };
        Self {
            standby: is_set(raw, 7),
            suspend: is_set(raw, 6),
            active_off: is_set(raw, 5),
            display,
            srgb: is_set(raw, 2),
            pref_timing_mode: is_set(raw, 1),
            continuous: is_set(raw, 0),
        }
    }

    /// DPMS standby supported.
    #[must_use]
    pub const fn standby(&self) -> bool {
        self.standby
    }

    /// DPMS suspend supported.
    #[must_use]
    pub const fn suspend(&self) -> bool {
        self.suspend
    }

    /// DPMS active-off supported.
    #[must_use]
    pub const fn active_off(&self) -> bool {
        self.active_off
    }

    /// Display type.
    #[must_use]
    pub const fn display(&self) -> DisplayType {
        self.display
    }

    /// Standard sRGB colour space. Bytes 25–34 must contain sRGB standard values.
    #[must_use]
    pub const fn srgb(&self) -> bool {
        self.srgb
    }

    /// Preferred timing mode specified in descriptor block 1.
    /// For EDID 1.3+ the preferred timing mode is always in the first Detailed Timing Descriptor.
    /// In that case, this bit specifies whether the preferred timing mode includes native pixel format and refresh rate.
    #[must_use]
    pub const fn pref_timing_mode(&self) -> bool {
        self.pref_timing_mode
    }

    /// Continuous timings with GTF or CVT.
    #[must_use]
    pub const fn continuous(&self) -> bool {
        self.continuous
    }
}
