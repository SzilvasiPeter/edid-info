//! CTA Immersive Audio Room Configuration.
//!
//! Extended data block (tag 7, extended tag 13) that describes
//! speaker layout and room geometry for immersive audio setups.
//!
//! # Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 0    | Extended tag (13) |
//! | 1    | Revision |
//! | 2    | Configuration flags |
//! | 3–5  | Speaker allocation |
//! | 6–8  | Farthest listener coordinates (x, y, z) |
//! | 9–11 | Display coordinates (x, y, z) |

use crate::edid::cta::speaker::{Speaker, SpeakerAlloc};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoomCfg {
    raw: u8,
}

impl RoomCfg {
    #[must_use]
    pub const fn parse(raw: u8) -> Self {
        Self { raw }
    }
    #[must_use]
    pub const fn display_valid(&self) -> bool {
        (self.raw & 0b1000_0000) != 0
    }
    #[must_use]
    pub const fn speaker_count_valid(&self) -> bool {
        (self.raw & 0b0100_0000) != 0
    }
    #[must_use]
    pub const fn sld_present(&self) -> bool {
        (self.raw & 0b0010_0000) != 0
    }
    #[must_use]
    pub const fn speaker_count(&self) -> Option<u8> {
        if self.speaker_count_valid() {
            Some((self.raw & 0b0001_1111) + 1)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coords3 {
    x: u8,
    y: u8,
    z: u8,
}

impl Coords3 {
    #[must_use]
    pub const fn parse(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }
    #[must_use]
    pub const fn x(&self) -> u8 {
        self.x
    }
    #[must_use]
    pub const fn y(&self) -> u8 {
        self.y
    }
    #[must_use]
    pub const fn z(&self) -> u8 {
        self.z
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoomConfig {
    rev: u8,
    cfg: RoomCfg,
    spm: SpeakerAlloc,
    far: Coords3,
    disp: Coords3,
}

impl RoomConfig {
    #[must_use]
    pub fn parse(raw: &[u8]) -> Option<Self> {
        if raw.len() < 12 || raw[0] != 13 {
            return None;
        }
        Some(Self {
            rev: raw[1],
            cfg: RoomCfg::parse(raw[2]),
            spm: SpeakerAlloc::parse(raw[3], raw[4], raw[5]),
            far: Coords3::parse(raw[6], raw[7], raw[8]),
            disp: Coords3::parse(raw[9], raw[10], raw[11]),
        })
    }
    #[must_use]
    pub const fn rev(&self) -> u8 {
        self.rev
    }
    #[must_use]
    pub const fn cfg(&self) -> RoomCfg {
        self.cfg
    }
    #[must_use]
    pub const fn has(&self, spk: Speaker) -> bool {
        self.spm.has(spk)
    }
    #[must_use]
    pub const fn far_raw(&self) -> Coords3 {
        self.far
    }
    #[must_use]
    pub const fn disp_raw(&self) -> Coords3 {
        self.disp
    }
    #[must_use]
    pub const fn far_x(&self) -> Option<u8> {
        if self.cfg.sld_present() && self.far.x != 0 {
            Some(self.far.x)
        } else {
            None
        }
    }
    #[must_use]
    pub const fn far_y(&self) -> Option<u8> {
        if self.cfg.sld_present() && self.far.y != 0 {
            Some(self.far.y)
        } else {
            None
        }
    }
    #[must_use]
    pub const fn far_z(&self) -> Option<u8> {
        if self.cfg.sld_present() && self.far.z != 0 {
            Some(self.far.z)
        } else {
            None
        }
    }
    #[must_use]
    pub const fn disp_x(&self) -> Option<u8> {
        if self.cfg.display_valid() && self.disp.x != 0 {
            Some(self.disp.x)
        } else {
            None
        }
    }
    #[must_use]
    pub const fn disp_y(&self) -> Option<u8> {
        if self.cfg.display_valid() && self.disp.y != 0 {
            Some(self.disp.y)
        } else {
            None
        }
    }
    #[must_use]
    pub const fn disp_z(&self) -> Option<u8> {
        if self.cfg.display_valid() && self.disp.z != 0 {
            Some(self.disp.z)
        } else {
            None
        }
    }
}
