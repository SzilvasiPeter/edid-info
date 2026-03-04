use crate::edid::base::BASE_LEN;

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
    pub fn parse_base(raw: &[u8; BASE_LEN]) -> Self {
        let mut out = [0; BASIC_LEN];
        out.copy_from_slice(&raw[BASIC_OFF..BASIC_OFF + BASIC_LEN]);
        Self::parse(&out)
    }

    #[must_use]
    pub const fn parse(raw: &[u8; BASIC_LEN]) -> Self {
        let input = VideoInput::parse(raw[0]);
        let is_digital = match input.kind() {
            InputKind::Digital { .. } => true,
            InputKind::Analog { .. } => false,
        };
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

    /// TODO: Check if 255, then gamma is defined by DI-EXT block. If bad ext-block, return None.
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
        let kind = if (raw & 0b1000_0000) != 0 {
            let depth = match raw & 0b0111_0000 {
                0b0000_0000 => BitDepth::Undef,
                0b0001_0000 => BitDepth::B6,
                0b0010_0000 => BitDepth::B8,
                0b0011_0000 => BitDepth::B10,
                0b0100_0000 => BitDepth::B12,
                0b0101_0000 => BitDepth::B14,
                0b0110_0000 => BitDepth::B16,
                _ => BitDepth::Reserved,
            };
            let iface = match raw & 0b0000_1111 {
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
            let level = match raw & 0b0110_0000 {
                0b0000_0000 => Level::V700_300,
                0b0010_0000 => Level::V714_286,
                0b0100_0000 => Level::V1000_400,
                _ => Level::V700_000,
            };
            InputKind::Analog {
                level,
                setup: (raw & 0b0001_0000) != 0,
                sep: (raw & 0b0000_1000) != 0,
                comp: (raw & 0b0000_0100) != 0,
                sog: (raw & 0b0000_0010) != 0,
                serr: (raw & 0b0000_0001) != 0,
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
            DisplayType::Digital(match raw & 0b0001_1000 {
                0b0000_0000 => DigitalType::Rgb444,
                0b0000_1000 => DigitalType::Rgb444Y444,
                0b0001_0000 => DigitalType::Rgb444Y422,
                _ => DigitalType::Rgb444Y444Y422,
            })
        } else {
            DisplayType::Analog(match raw & 0b0001_1000 {
                0b0000_0000 => AnalogType::MonoGray,
                0b0000_1000 => AnalogType::Rgb,
                0b0001_0000 => AnalogType::NonRgb,
                _ => AnalogType::Undef,
            })
        };
        Self {
            stand: (raw & 0b1000_0000) != 0,
            susp: (raw & 0b0100_0000) != 0,
            off: (raw & 0b0010_0000) != 0,
            display,
            srgb: (raw & 0b0000_0100) != 0,
            pref: (raw & 0b0000_0010) != 0,
            cont: (raw & 0b0000_0001) != 0,
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
