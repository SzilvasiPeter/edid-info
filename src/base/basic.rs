//! Basic display parameters (bytes 20–24).
//!
//! # Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 20   | Video input parameters |
//! | 21   | Horizontal display size (cm) |
//! | 22   | Vertical display size (cm) |
//! | 23   | Gamma (value - 1.0, scaled by 100) |
//! | 24   | Feature support flags |

use crate::{
    bit::{get_bits, is_set},
    common::{BLOCK_LEN, FailureKind, Validation, WarningKind},
};

pub const BASIC_OFF: usize = 20;
pub const BASIC_LEN: usize = 5;

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

/// Video input type (Byte 20).
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
        /// Video white and sync levels, relative to blank (Bits 6–5 of Byte 20).
        level: Level,
        /// Blank-to-black setup (pedestal) expected (Bits 4 of Byte 20).
        blank_to_black: bool,
        /// Separate sync supported (Bits 3 of Byte 20).
        separate_sync: bool,
        /// Composite sync (on `HSync`) supported (Bits 2 of Byte 20).
        composite_sync: bool,
        /// Sync on green supported (Bits 1 of Byte 20).
        sync_on_green: bool,
        /// `VSync` pulse must be serrated when composite or sync-on-green is used (Bits 0 of Byte 20).
        serrated_sync: bool,
    },
}

/// Analog display type.
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
    Undefined,
}

/// Digital display type.
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

/// Display type (analog or digital) for features bitmap (Bits 4-3 of Byte 24).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayType {
    Digital(DigitalType),
    Analog(AnalogType),
}

/// Basic display parameters containing video input parameters, screen information, and supported features.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Basic {
    input: u8,
    width_cm: u8,
    height_cm: u8,
    gamma: u8,
    features: u8,
}

impl Basic {
    /// Parses the basic information from base block bytes.
    ///
    /// Byte sizes:
    /// - `input`: 1 byte
    /// - `width_cm`: 1 byte
    /// - `height_cm`: 1 byte
    /// - `gamma`: 1 byte
    /// - `features`: 1 byte
    #[must_use]
    pub const fn parse(raw: &[u8; BLOCK_LEN]) -> Self {
        Self {
            input: raw[BASIC_OFF],
            width_cm: raw[BASIC_OFF + 1],
            height_cm: raw[BASIC_OFF + 2],
            gamma: raw[BASIC_OFF + 3],
            features: raw[BASIC_OFF + 4],
        }
    }

    /// Video input parameters bitmap.
    #[must_use]
    pub const fn input(&self) -> VideoInput {
        VideoInput::parse(self.input)
    }

    // TODO: Add field for landscape aspect ratio when height is zero
    /// Horizontal screen size, in centimetres (range 1–255).
    /// If vertical screen size is 0, landscape aspect ratio (range 1.00–3.54),
    /// datavalue = (AR×100) − 99 (example: 16:9, 79; 4:3, 34.)
    #[must_use]
    pub const fn width_cm(&self) -> u8 {
        self.width_cm
    }

    // TODO: Add field for portrait aspect ratio when width is zero
    /// Vertical screen size, in centimetres.
    /// If horizontal screen size is 0, portrait aspect ratio (range 0.28–0.99),
    /// datavalue = (100/AR) − 99 (example: 9:16, 79; 3:4, 34.)
    /// If both bytes are 0, screen size and aspect ratio are undefined (e.g. projector)
    #[must_use]
    pub const fn height_cm(&self) -> u8 {
        self.height_cm
    }

    /// Display gamma, factory default (range 1.00–3.54), as gamma × 100.
    /// If 255, gamma is defined by DI-EXT block.
    #[must_use]
    pub const fn gamma(&self) -> Option<u16> {
        if self.gamma == 255 {
            None
        } else {
            Some(self.gamma as u16 + 100)
        }
    }

    /// Supported features bitmap.
    #[must_use]
    pub const fn features(&self) -> Features {
        let is_digital = matches!(self.input().kind(), InputKind::Digital { .. });
        Features::parse(self.features, is_digital)
    }

    /// Validates the basic block.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        let mut validation = Validation::new();
        if let InputKind::Digital { depth, iface } = self.input().kind() {
            validation = validation.fail_if(
                matches!(depth, BitDepth::Reserved),
                FailureKind::BasicColorDepthReserved,
            );
            validation = validation.fail_if(
                matches!(iface, Interface::Other(_)),
                FailureKind::BasicInterfaceReserved,
            );
        }

        let gamma_warn = match self.gamma() {
            Some(value) => value != 220,
            None => false,
        };
        validation = validation.warn_if(
            self.features().standard_rgb() && gamma_warn,
            WarningKind::BasicSrgbGammaInvalid,
        );
        validation = validation.warn_if(
            self.width_cm != 0
                && self.height_cm != 0
                && (self.width_cm < 10 || self.height_cm < 10),
            WarningKind::BasicImageSizeDubious,
        );

        validation
    }
}

// TODO: Implement the Display trait for `Basic`

/// Video input parameters.
/// If Bit 7 (of Byte 20) is set, then digital, else analog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VideoInput {
    kind: InputKind,
}

impl VideoInput {
    /// Parses video input parameters from Byte 20.
    ///
    /// Bit fields:
    /// - `bit 7`: 0=Analog, 1=Digital
    ///
    /// If **Digital**:
    /// - `bits 6–4`: Color bit depth
    /// - `bits 3–0`: Video interface
    ///
    /// If **Analog**:
    /// - `bits 6–5`: Video white and sync levels
    /// - `bit 4`: Blank-to-black setup (pedestal)
    /// - `bit 3`: Separate sync
    /// - `bit 2`: Composite sync
    /// - `bit 1`: Sync on green
    /// - `bit 0`: Serrated sync
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
                blank_to_black: is_set(raw, 4),
                separate_sync: is_set(raw, 3),
                composite_sync: is_set(raw, 2),
                sync_on_green: is_set(raw, 1),
                serrated_sync: is_set(raw, 0),
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

// TODO: Implement the Display trait for `VideoInput`

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
///
/// # References
/// - [DPMS](https://en.wikipedia.org/wiki/VESA_Display_Power_Management_Signaling) (Display Power Management Signaling)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[expect(clippy::struct_excessive_bools, reason = "Spec-aligned EDID bitfields")]
pub struct Features {
    standby: bool,
    suspend: bool,
    active_off: bool,
    display: DisplayType,
    standard_rgb: bool,
    timing_mode: bool,
    timing_continuous: bool,
}

impl Features {
    /// Parses feature flags from the features byte.
    ///
    /// Fields (Byte 24):
    /// - `standby`: bit 7, DPMS standby support
    /// - `suspend`: bit 6, DPMS suspend support
    /// - `active_off`: bit 5, DPMS active-off support
    /// - `display`: bits 4–3, analog or digital display type
    /// - `standard_rgb`: bit 2, standard sRGB colour space
    /// - `timing_mode`: bit 1, preferred timing mode
    /// - `timing_continuous`: bit 0, continuous timings with GTF or CVT
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
                _ => AnalogType::Undefined,
            })
        };
        Self {
            standby: is_set(raw, 7),
            suspend: is_set(raw, 6),
            active_off: is_set(raw, 5),
            display,
            standard_rgb: is_set(raw, 2),
            timing_mode: is_set(raw, 1),
            timing_continuous: is_set(raw, 0),
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
    pub const fn standard_rgb(&self) -> bool {
        self.standard_rgb
    }

    /// Preferred timing mode specified in descriptor block 1.
    /// For EDID 1.3+ the preferred timing mode is always in the first Detailed Timing Descriptor.
    /// In that case, this bit specifies whether the preferred timing mode includes native pixel format and refresh rate.
    #[must_use]
    pub const fn timing_mode(&self) -> bool {
        self.timing_mode
    }

    /// Continuous timings with GTF or CVT.
    #[must_use]
    pub const fn timing_continuous(&self) -> bool {
        self.timing_continuous
    }
}

// TODO: Implement the Display trait for `Features`
