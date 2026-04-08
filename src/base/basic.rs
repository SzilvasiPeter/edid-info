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
    common::{AspectRatio, BLOCK_LEN, FailureKind, Validation, WarningKind},
};

/// Basic display information offset in the base block.
pub const BASIC_OFF: usize = 20;

/// Basic display information length in bytes.
pub const BASIC_LEN: usize = 5;

/// Bit depth (Byte 20).
///
/// | Bits (6-4) | Description |
/// |------------|-------------|
/// | 000        | undefined |
/// | 001        | 6 bits per color |
/// | 010        | 8 bits per color |
/// | 011        | 10 bits per color |
/// | 100        | 12 bits per color |
/// | 101        | 14 bits per color |
/// | 110        | 16 bits per color |
/// | 111        | reserved |
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

/// Video interface (Byte 20).
///
/// | Bits 0-3 | Description |
/// |----------|-------------|
/// | 0000     | undefined |
/// | 0001     | DVI |
/// | 0010     | HDMIa |
/// | 0011     | HDMIb |
/// | 0100     | MDDI |
/// | 0101     | DisplayPort |
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

/// Video white and sync levels, relative to blank (Byte 20).
///
/// | Bits 5-6 | Voltage Levels |
/// |----------|----------------|
/// | 00       | +0.7/−0.3 V |
/// | 01       | +0.714/−0.286 V |
/// | 10       | +1.0/−0.4 V |
/// | 11       | +0.7/0 V (EVC) |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Level {
    V700_300,
    V714_286,
    V1000_400,
    V700_000,
}

/// Video input type (Byte 20).
///
/// | Bit 7 | Description |
/// |-------|-------------|
/// | 1     | Digital input |
/// | 0     | Analog input |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputKind {
    /// Digital input with bit depth and interface.
    Digital { depth: BitDepth, iface: Interface },
    /// Analog input with signal level and sync options.
    Analog {
        /// Video white and sync levels, relative to blank.
        level: Level,
        /// Blank-to-black setup (pedestal) expected.
        blank_to_black: bool,
        /// Separate sync supported: `HSync` and `VSync` on dedicated wires.
        separate_sync: bool,
        /// Composite sync supported: `HSync` and `VSync` combined on one wire.
        composite_sync: bool,
        /// Sync-on-Green supported: sync signals modulated onto the green video channel.
        sync_on_green: bool,
        /// Serrated `VSync` supported: required when using composite or sync-on-green.
        ///
        /// Serration adds notches to the `VSync` pulse so the display can distinguish it
        /// from a long `HSync` pulse when both signals share a single wire or channel.
        serrated_sync: bool,
    },
}

/// Analog display type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalogType {
    MonoGray,
    Rgb,
    NonRgb,
    Undefined,
}

/// Digital display type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DigitalType {
    Rgb444,
    Rgb444Y444,
    Rgb444Y422,
    Rgb444Y444Y422,
}

/// Display type (analog or digital) for features bitmap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayType {
    Digital(DigitalType),
    Analog(AnalogType),
}

impl DisplayType {
    /// Parses display type from bits 3-4 of Byte 24.
    ///
    /// Analog mapping:
    /// | Bits 3-4 | Description |
    /// |----------|-------------|
    /// | 00       | monochrome or grayscale |
    /// | 01       | RGB color |
    /// | 10       | non-RGB color |
    /// | 11       | undefined |
    ///
    /// Digital mapping:
    /// | Bits 3-4 | Description |
    /// |----------|-------------|
    /// | 00       | RGB 4:4:4 |
    /// | 01       | RGB 4:4:4 + YCrCb 4:4:4 |
    /// | 10       | RGB 4:4:4 + YCrCb 4:2:2 |
    /// | 11       | RGB 4:4:4 + YCrCb 4:4:4 + YCrCb 4:2:2 |
    #[must_use]
    pub const fn parse(bits: u8, is_digital: bool) -> Self {
        if is_digital {
            match bits {
                0b00 => Self::Digital(DigitalType::Rgb444),
                0b01 => Self::Digital(DigitalType::Rgb444Y444),
                0b10 => Self::Digital(DigitalType::Rgb444Y422),
                _ => Self::Digital(DigitalType::Rgb444Y444Y422),
            }
        } else {
            match bits {
                0b00 => Self::Analog(AnalogType::MonoGray),
                0b01 => Self::Analog(AnalogType::Rgb),
                0b10 => Self::Analog(AnalogType::NonRgb),
                _ => Self::Analog(AnalogType::Undefined),
            }
        }
    }
}

/// Screen size interpretation according to EDID spec.
///
/// The `width_cm` and `height_cm` raw bytes can represent:
/// - Physical dimensions when both are non-zero
/// - Landscape aspect ratio when height is zero
/// - Portrait aspect ratio when width is zero
/// - Undefined when both are zero (e.g. projector)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreenSize {
    /// Physical screen dimensions in centimetres.
    Dimensions { width_cm: u8, height_cm: u8 },
    /// Landscape aspect ratio:
    /// `width = raw_byte + 99`, `height = 100`.
    Landscape(AspectRatio),
    /// Portrait aspect ratio:
    /// `width = 100`, `height = raw_byte + 99`.
    Portrait(AspectRatio),
    /// Both bytes are zero, screen size is undefined (e.g. projector).
    Undefined,
}

/// Basic display parameters containing video input parameters, screen information, and supported features.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Basic {
    video_input: u8,
    width_cm: u8,
    height_cm: u8,
    gamma: u8,
    features: u8,
}

impl Basic {
    /// Parses the basic information from base block bytes.
    ///
    /// Byte sizes:
    /// - `video_input`: 1 byte
    /// - `width_cm`: 1 byte
    /// - `height_cm`: 1 byte
    /// - `gamma`: 1 byte
    /// - `features`: 1 byte
    #[must_use]
    pub fn new(raw: &[u8; BLOCK_LEN]) -> Self {
        let basic = &raw[BASIC_OFF..BASIC_OFF + BASIC_LEN];
        Self {
            video_input: basic[0],
            width_cm: basic[1],
            height_cm: basic[2],
            gamma: basic[3],
            features: basic[4],
        }
    }

    /// Video input parameters bitmap.
    #[must_use]
    pub const fn video_input(&self) -> VideoInput {
        VideoInput::parse(self.video_input)
    }

    /// Returns [`ScreenSize`] interpretation based on the width and height:
    /// - Both non-zero: [`ScreenSize::Dimensions`]
    /// - Height zero: [`ScreenSize::Landscape`] with `width = width_cm + 99`, `height = 100`
    /// - Width zero: [`ScreenSize::Portrait`] with `width = 100`, `height = height_cm + 99`
    /// - Both zero: [`ScreenSize::Undefined`]
    #[must_use]
    pub const fn screen_size(&self) -> ScreenSize {
        match (self.width_cm, self.height_cm) {
            (0, 0) => ScreenSize::Undefined,
            (w, 0) => ScreenSize::Landscape(AspectRatio {
                width: w as u16 + 99,
                height: 100,
            }),
            (0, h) => ScreenSize::Portrait(AspectRatio {
                width: 100,
                height: h as u16 + 99,
            }),
            (w, h) => ScreenSize::Dimensions {
                width_cm: w,
                height_cm: h,
            },
        }
    }

    /// Display gamma as gamma × 100 (factory default, 1.00–3.54).
    /// Returns `None` when the raw value is `0xFF` (defined by an extension block).
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
        let is_digital = matches!(self.video_input().kind(), InputKind::Digital { .. });
        Features::parse(self.features, is_digital)
    }

    /// Validates the basic block.
    #[must_use]
    pub const fn validate(&self, chroma_srgb: bool) -> Validation {
        let (bitdepth_reserved, iface_reserved) = match self.video_input().kind() {
            InputKind::Digital { depth, iface } => (
                matches!(depth, BitDepth::Reserved),
                matches!(iface, Interface::Other(_)),
            ),
            InputKind::Analog { .. } => (false, false),
        };
        let srgb = self.features().standard_rgb();
        let gamma_warn = match self.gamma() {
            Some(value) => value != 220,
            None => false,
        };
        let small_size = self.width_cm != 0
            && self.height_cm != 0
            && (self.width_cm < 10 || self.height_cm < 10);
        Validation::new()
            .fail_if(bitdepth_reserved, FailureKind::BasicColorDepthReserved)
            .fail_if(iface_reserved, FailureKind::BasicInterfaceReserved)
            .fail_if(srgb && !chroma_srgb, FailureKind::BasicSrgbChromaMismatch)
            .warn_if(!srgb && chroma_srgb, WarningKind::BasicSrgbNotSignaled)
            .warn_if(srgb && gamma_warn, WarningKind::BasicSrgbGammaInvalid)
            .warn_if(small_size, WarningKind::BasicImageSizeDubious)
    }
}

impl core::fmt::Display for Basic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let input = self.video_input();
        let size = self.screen_size();
        let features = self.features();
        write!(f, "Input: {input}, ")?;
        match size {
            ScreenSize::Dimensions {
                width_cm,
                height_cm,
            } => {
                write!(f, "Size: {width_cm}x{height_cm} cm")?;
            }
            ScreenSize::Landscape(ratio) => {
                write!(f, "Aspect: {}:{} landscape", ratio.width, ratio.height)?;
            }
            ScreenSize::Portrait(ratio) => {
                write!(f, "Aspect: {}:{} portrait", ratio.width, ratio.height)?;
            }
            ScreenSize::Undefined => {
                write!(f, "Size: undefined")?;
            }
        }
        write!(f, ", ")?;
        match self.gamma() {
            Some(value) => {
                let major = value / 100;
                let minor = value % 100;
                write!(f, "Gamma: {major}.{minor:02}")?;
            }
            None => {
                write!(f, "Gamma is defined by an extension")?;
            }
        }
        write!(f, ", Features: {features}")
    }
}

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
    /// - bit 7: Digital flag
    ///
    /// If **Digital**:
    /// - bits 4-6: Color bit depth
    /// - bits 0-3: Video interface
    ///
    /// If **Analog**:
    /// - bits 5-6: Video white and sync levels
    /// - bit 4: Blank-to-black setup (pedestal) expected
    /// - bit 3: Separate sync supported
    /// - bit 2: Composite sync supported
    /// - bit 1: Sync on green supported
    /// - bit 0: Serrated sync supported
    #[must_use]
    pub const fn parse(video_input: u8) -> Self {
        let kind = if is_set(video_input, 7) {
            let depth = match get_bits(video_input, 4, 6) {
                0b000 => BitDepth::Undef,
                0b001 => BitDepth::B6,
                0b010 => BitDepth::B8,
                0b011 => BitDepth::B10,
                0b100 => BitDepth::B12,
                0b101 => BitDepth::B14,
                0b110 => BitDepth::B16,
                _ => BitDepth::Reserved,
            };
            let iface = match get_bits(video_input, 0, 3) {
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
            let level = match get_bits(video_input, 5, 6) {
                0b00 => Level::V700_300,
                0b01 => Level::V714_286,
                0b10 => Level::V1000_400,
                _ => Level::V700_000,
            };
            InputKind::Analog {
                level,
                blank_to_black: is_set(video_input, 4),
                separate_sync: is_set(video_input, 3),
                composite_sync: is_set(video_input, 2),
                sync_on_green: is_set(video_input, 1),
                serrated_sync: is_set(video_input, 0),
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

impl core::fmt::Display for VideoInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind() {
            InputKind::Digital { depth, iface } => {
                let depth = match depth {
                    BitDepth::Undef => "undefined",
                    BitDepth::B6 => "6 bit per color",
                    BitDepth::B8 => "8 bit per color",
                    BitDepth::B10 => "10 bit per color",
                    BitDepth::B12 => "12 bit per color",
                    BitDepth::B14 => "14 bit per color",
                    BitDepth::B16 => "16 bit per color",
                    BitDepth::Reserved => "reserved",
                };
                let iface = match iface {
                    Interface::Undef => "undefined",
                    Interface::Dvi => "DVI",
                    Interface::HdmiA => "HDMIa",
                    Interface::HdmiB => "HDMIb",
                    Interface::Mddi => "MDDI",
                    Interface::DisplayPort => "DisplayPort",
                    Interface::Other(value) => {
                        return write!(f, "Digital (depth: {depth}, iface: 0x{value:01X})");
                    }
                };
                write!(f, "Digital (depth: {depth}, iface: {iface})")
            }
            InputKind::Analog {
                level,
                blank_to_black,
                separate_sync,
                composite_sync,
                sync_on_green,
                serrated_sync,
            } => {
                let level = match level {
                    Level::V700_300 => "+0.7/-0.3 V",
                    Level::V714_286 => "+0.714/-0.286 V",
                    Level::V1000_400 => "+1.0/-0.4 V",
                    Level::V700_000 => "+0.7/0 V",
                };
                write!(
                    f,
                    "Analog (level: {level}, blank: {blank_to_black}, separate: {separate_sync}, composite: {composite_sync}, green: {sync_on_green}, serrated: {serrated_sync})"
                )
            }
        }
    }
}

/// DPMS support flags.
///
/// | Bit | Description |
/// |-----|-------------|
/// | 7 | DPMS standby supported |
/// | 6 | DPMS suspend supported |
/// | 5 | DPMS active-off supported |
///
/// # References
/// - [DPMS](https://en.wikipedia.org/wiki/VESA_Display_Power_Management_Signaling) (Display Power Management Signaling)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dpms {
    pub standby: bool,
    pub suspend: bool,
    pub active_off: bool,
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
pub struct Features {
    dpms: Dpms,
    display: DisplayType,
    standard_rgb: bool,
    timing_mode: bool,
    timing_continuous: bool,
}

impl Features {
    /// Parses feature flags from the features byte.
    ///
    /// Fields (Byte 24):
    /// - `dpms`: bits 7–5, DPMS support flags
    /// - `display`: bits 4–3, analog or digital display type
    /// - `standard_rgb`: bit 2, standard sRGB colour space
    /// - `timing_mode`: bit 1, preferred timing mode
    /// - `timing_continuous`: bit 0, continuous timings with GTF or CVT
    #[must_use]
    pub const fn parse(features: u8, digital: bool) -> Self {
        Self {
            dpms: Dpms {
                standby: is_set(features, 7),
                suspend: is_set(features, 6),
                active_off: is_set(features, 5),
            },
            display: DisplayType::parse(get_bits(features, 3, 4), digital),
            standard_rgb: is_set(features, 2),
            timing_mode: is_set(features, 1),
            timing_continuous: is_set(features, 0),
        }
    }

    /// DPMS support flags (standby, suspend and active-off).
    #[must_use]
    pub const fn dpms(&self) -> Dpms {
        self.dpms
    }

    /// Display type.
    #[must_use]
    pub const fn display(&self) -> DisplayType {
        self.display
    }

    /// [Standard RGB](https://en.wikipedia.org/wiki/SRGB) colour space.
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

impl core::fmt::Display for Features {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let display = match self.display {
            DisplayType::Digital(kind) => {
                let label = match kind {
                    DigitalType::Rgb444 => "RGB 4:4:4",
                    DigitalType::Rgb444Y444 => "RGB 4:4:4 + YCrCb 4:4:4",
                    DigitalType::Rgb444Y422 => "RGB 4:4:4 + YCrCb 4:2:2",
                    DigitalType::Rgb444Y444Y422 => "RGB 4:4:4 + YCrCb 4:4:4 + YCrCb 4:2:2",
                };
                ("Digital", label)
            }
            DisplayType::Analog(kind) => {
                let label = match kind {
                    AnalogType::MonoGray => "monochrome/gray",
                    AnalogType::Rgb => "RGB",
                    AnalogType::NonRgb => "non-RGB",
                    AnalogType::Undefined => "undefined",
                };
                ("Analog", label)
            }
        };
        write!(
            f,
            "DPMS: standby={}, suspend={}, off={}, Display: {} ({}), sRGB: {}, Preferred timing: {}, Continuous: {}",
            self.dpms.standby,
            self.dpms.suspend,
            self.dpms.active_off,
            display.0,
            display.1,
            self.standard_rgb,
            self.timing_mode,
            self.timing_continuous
        )
    }
}
