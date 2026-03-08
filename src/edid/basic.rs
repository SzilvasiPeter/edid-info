use crate::edid::BLOCK_LEN;
use crate::edid::bits::{get_bits, is_set};

pub const BASIC_OFF: usize = 20;
pub const BASIC_LEN: usize = 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Basic {
    input: VideoInput,
    width_cm: u8,
    height_cm: u8,
    gamma: u8,
    feat: Features,
}

impl Basic {
    #[must_use]
    pub fn parse_base(raw: &[u8; BLOCK_LEN]) -> Self {
        let mut out = [0; BASIC_LEN];
        out.copy_from_slice(&raw[BASIC_OFF..BASIC_OFF + BASIC_LEN]);
        Self::parse(&out)
    }

    #[must_use]
    pub const fn parse(raw: &[u8; BASIC_LEN]) -> Self {
        let input = VideoInput::parse(raw[0]);
        let is_digital = matches!(input.kind(), InputKind::Digital { .. });
        Self {
            input,
            width_cm: raw[1],
            height_cm: raw[2],
            gamma: raw[3],
            feat: Features::parse(raw[4], is_digital),
        }
    }

    #[must_use]
    pub const fn input(&self) -> VideoInput {
        self.input
    }

    #[must_use]
    pub const fn width_cm(&self) -> u8 {
        self.width_cm
    }

    #[must_use]
    pub const fn height_cm(&self) -> u8 {
        self.height_cm
    }

    #[must_use]
    pub const fn gamma_raw(&self) -> u8 {
        self.gamma
    }

    #[must_use]
    pub const fn feat(&self) -> Features {
        self.feat
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputKind {
    Digital {
        depth: BitDepth,
        iface: Interface,
    },
    Analog {
        level: Level,
        setup: bool,
        sep: bool,
        comp: bool,
        sog: bool,
        serr: bool,
    },
}

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

    #[must_use]
    pub const fn kind(&self) -> InputKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayType {
    Digital(DigitalType),
    Analog(AnalogType),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalogType {
    MonoGray,
    Rgb,
    NonRgb,
    Undef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DigitalType {
    Rgb444,
    Rgb444Y444,
    Rgb444Y422,
    Rgb444Y444Y422,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[expect(clippy::struct_excessive_bools, reason = "Spec-aligned EDID bitfields")]
pub struct Features {
    stand: bool,
    susp: bool,
    off: bool,
    display: DisplayType,
    srgb: bool,
    pref: bool,
    cont: bool,
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
            stand: is_set(raw, 7),
            susp: is_set(raw, 6),
            off: is_set(raw, 5),
            display,
            srgb: is_set(raw, 2),
            pref: is_set(raw, 1),
            cont: is_set(raw, 0),
        }
    }

    #[must_use]
    pub const fn stand(&self) -> bool {
        self.stand
    }

    #[must_use]
    pub const fn susp(&self) -> bool {
        self.susp
    }

    #[must_use]
    pub const fn off(&self) -> bool {
        self.off
    }

    #[must_use]
    pub const fn display(&self) -> DisplayType {
        self.display
    }

    #[must_use]
    pub const fn srgb(&self) -> bool {
        self.srgb
    }

    #[must_use]
    pub const fn pref(&self) -> bool {
        self.pref
    }

    #[must_use]
    pub const fn cont(&self) -> bool {
        self.cont
    }
}
