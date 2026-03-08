use crate::edid::DESC_LEN;
use crate::edid::bits::{u6_pack, u10_lo, u12_hi, u12_lo};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DetailedTiming {
    pixel_clock_hz: u32,
    h_active: u16,
    h_blank: u16,
    v_active: u16,
    v_blank: u16,
    h_front: u16,
    h_sync: u16,
    v_front: u8,
    v_sync: u8,
    h_size_mm: u16,
    v_size_mm: u16,
    h_border: u8,
    v_border: u8,
    feat: Features,
}

impl DetailedTiming {
    #[must_use]
    pub fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        let clk = u32::from(u16::from_le_bytes([raw[0], raw[1]])) * CLK_UNIT;
        if clk == 0 {
            return None;
        }
        Some(Self {
            pixel_clock_hz: clk,
            h_active: u12_hi(raw[2], raw[4]),
            h_blank: u12_lo(raw[3], raw[4]),
            v_active: u12_hi(raw[5], raw[7]),
            v_blank: u12_lo(raw[6], raw[7]),
            h_front: u10_lo(raw[8], u16::from(raw[11] >> 6)),
            h_sync: u10_lo(raw[9], u16::from((raw[11] >> 4) & 0x03)),
            v_front: u6_pack((raw[10] >> 4) & 0x0f, (raw[11] >> 2) & 0x03),
            v_sync: u6_pack(raw[10] & 0x0f, raw[11] & 0x03),
            h_size_mm: u12_hi(raw[12], raw[14]),
            v_size_mm: u12_lo(raw[13], raw[14]),
            h_border: raw[15],
            v_border: raw[16],
            feat: parse_feat(raw[17]),
        })
    }

    #[must_use]
    pub const fn pixel_clock_hz(&self) -> u32 {
        self.pixel_clock_hz
    }
    #[must_use]
    pub const fn h_active(&self) -> u16 {
        self.h_active
    }
    #[must_use]
    pub const fn h_blank(&self) -> u16 {
        self.h_blank
    }
    #[must_use]
    pub const fn v_active(&self) -> u16 {
        self.v_active
    }
    #[must_use]
    pub const fn v_blank(&self) -> u16 {
        self.v_blank
    }
    #[must_use]
    pub const fn h_front(&self) -> u16 {
        self.h_front
    }
    #[must_use]
    pub const fn h_sync(&self) -> u16 {
        self.h_sync
    }
    #[must_use]
    pub const fn h_back(&self) -> u16 {
        self.h_blank - self.h_front - self.h_sync
    }
    #[must_use]
    pub const fn v_front(&self) -> u8 {
        self.v_front
    }
    #[must_use]
    pub const fn v_sync(&self) -> u8 {
        self.v_sync
    }
    #[must_use]
    pub fn v_back(&self) -> u16 {
        self.v_blank - u16::from(self.v_front) - u16::from(self.v_sync)
    }
    #[must_use]
    pub fn h_khz(&self) -> f64 {
        f64::from(self.pixel_clock_hz) / f64::from(self.h_active + self.h_blank) / 1000.0
    }
    #[must_use]
    pub fn v_hz(&self) -> f64 {
        f64::from(self.pixel_clock_hz)
            / f64::from(self.h_active + self.h_blank)
            / f64::from(self.v_active + self.v_blank)
    }
    #[must_use]
    pub const fn h_size_mm(&self) -> u16 {
        self.h_size_mm
    }
    #[must_use]
    pub const fn v_size_mm(&self) -> u16 {
        self.v_size_mm
    }
    #[must_use]
    pub const fn h_border(&self) -> u8 {
        self.h_border
    }
    #[must_use]
    pub const fn v_border(&self) -> u8 {
        self.v_border
    }
    #[must_use]
    pub const fn feat(&self) -> Features {
        self.feat
    }
}

const fn parse_feat(raw: u8) -> Features {
    Features {
        interlaced: (raw & 0b1000_0000) != 0,
        stereo: parse_stereo(raw),
        sync: parse_sync(raw),
    }
}

const fn parse_stereo(raw: u8) -> Stereo {
    match (raw >> 5, raw & 0x01) {
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
    match (raw >> 3) & 0x03 {
        0b00 | 0b01 => Sync::Analog {
            bipolar: (raw & 0b0000_1000) != 0,
            serr: (raw & 0b0000_0100) != 0,
            rgb: (raw & 0b0000_0010) != 0,
        },
        0b10 => Sync::DigitalComposite {
            serr: (raw & 0b0000_0100) != 0,
            h_polar: (raw & 0b0000_0010) != 0,
        },
        _ => Sync::DigitalSeparate {
            v_polar: (raw & 0b0000_0100) != 0,
            h_polar: (raw & 0b0000_0010) != 0,
        },
    }
}
