//! CVT 3-Byte Timing Codes Descriptor.
//!
//! Compact timing descriptors using the Coordinated Video Timing
//! standard. Each 3-byte entry encodes vertical lines, aspect ratio,
//! and supported refresh rates. Uses tag 0xF8 and version 0x01.
//!
//! # CVT 3-Byte Entry Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 0–1  | Vertical lines / 2 - 1 (11 bits) + aspect ratio (2 bits) |
//! | 2    | Refresh rate flags + preferred rate |

use crate::common::AspectRatio;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrefRate {
    Hz50,
    Hz60,
    Hz75,
    Hz85,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[expect(clippy::struct_excessive_bools, reason = "Spec-aligned EDID bitfields")]
pub struct Mode {
    addr_lines: u16,
    aspect: AspectRatio,
    // TODO: Use a single Frequency enum for the bitmaps
    // VerticalRate or RefreshRate or VerticalFrequency
    pref: PrefRate,
    // [Option<VerticalRate>; 5]
    hz50: bool,
    hz60: bool,
    hz75: bool,
    hz85: bool,
    hz60_rb: bool,
}

impl Mode {
    fn parse(raw: [u8; 3]) -> Self {
        let addr_lines = (u16::from(raw[0])) | (u16::from((raw[1] >> 4) & 0x0F) << 8);
        let aspect = match (raw[1] >> 2) & 0b11 {
            0b00 => AspectRatio::new(4, 3),
            0b01 => AspectRatio::new(16, 9),
            0b10 => AspectRatio::new(16, 10),
            _ => AspectRatio::new(15, 9),
        };
        let pref = match (raw[2] >> 5) & 0b11 {
            0b00 => PrefRate::Hz50,
            0b01 => PrefRate::Hz60,
            0b10 => PrefRate::Hz75,
            _ => PrefRate::Hz85,
        };
        Self {
            addr_lines,
            aspect,
            pref,
            hz50: (raw[2] & 0b0001_0000) != 0,
            hz60: (raw[2] & 0b0000_1000) != 0,
            hz75: (raw[2] & 0b0000_0100) != 0,
            hz85: (raw[2] & 0b0000_0010) != 0,
            hz60_rb: (raw[2] & 0b0000_0001) != 0,
        }
    }

    #[must_use]
    pub const fn addr_lines(&self) -> u16 {
        self.addr_lines
    }
    #[must_use]
    pub const fn aspect(&self) -> AspectRatio {
        self.aspect
    }
    #[must_use]
    pub const fn pref(&self) -> PrefRate {
        self.pref
    }
    #[must_use]
    pub const fn hz50(&self) -> bool {
        self.hz50
    }
    #[must_use]
    pub const fn hz60(&self) -> bool {
        self.hz60
    }
    #[must_use]
    pub const fn hz75(&self) -> bool {
        self.hz75
    }
    #[must_use]
    pub const fn hz85(&self) -> bool {
        self.hz85
    }
    #[must_use]
    pub const fn hz60_rb(&self) -> bool {
        self.hz60_rb
    }

    #[must_use]
    pub const fn v_lines(&self) -> u16 {
        (self.addr_lines + 1) * 2
    }

    // TODO: Find real world data that could trigger the overflow
    #[must_use]
    pub const fn h_pixels(&self) -> u16 {
        let v = self.v_lines();
        let r = self.aspect;
        // Calculate (v * width / height) without intermediate overflow
        let h = (v / r.height()) * r.width() + (v % r.height()) * r.width() / r.height();
        (h / 8) * 8
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cvt3 {
    mode1: Mode,
    mode2: Mode,
    mode3: Mode,
    mode4: Mode,
}

impl Cvt3 {
    pub(super) fn parse(raw: &[u8; 13]) -> Self {
        Self {
            mode1: Mode::parse([raw[1], raw[2], raw[3]]),
            mode2: Mode::parse([raw[4], raw[5], raw[6]]),
            mode3: Mode::parse([raw[7], raw[8], raw[9]]),
            mode4: Mode::parse([raw[10], raw[11], raw[12]]),
        }
    }

    #[must_use]
    pub const fn mode1(&self) -> Mode {
        self.mode1
    }
    #[must_use]
    pub const fn mode2(&self) -> Mode {
        self.mode2
    }
    #[must_use]
    pub const fn mode3(&self) -> Mode {
        self.mode3
    }
    #[must_use]
    pub const fn mode4(&self) -> Mode {
        self.mode4
    }
}
