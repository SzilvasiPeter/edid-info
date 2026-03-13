//! Standard Timing Identification (bytes 38–53).
//!
//! Contains 8 timing descriptors, each 2 bytes, describing common
//! display resolutions and refresh rates.
//!
//! # Structure
//!
//! Each 2-byte entry encodes:
//! - Byte N: Horizontal resolution code ((width/8) - 31)
//! - Byte N+1: Aspect ratio (bits 7–6) + vertical refresh (bits 5–0)
//!
//! | Offset | Count | Description |
//! |--------|-------|-------------|
//! | 38–53  | 8×2   | Standard timing descriptors |
//!
//! If both bytes are 0x01, the entry is unused.

pub const STANDARD_OFF: usize = 38;
pub const STANDARD_LEN: usize = 16;
pub const STANDARD_NUM: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Aspect {
    A16_10,
    A4_3,
    A5_4,
    A16_9,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timing {
    width: u16,
    height: u16,
    aspect: Aspect,
    vfreq: u8,
}

impl Timing {
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }

    #[must_use]
    pub const fn aspect(&self) -> Aspect {
        self.aspect
    }

    #[must_use]
    pub const fn vfreq(&self) -> u8 {
        self.vfreq
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Std1 {
    modes: [Option<Timing>; STANDARD_NUM],
}

impl Std1 {
    #[must_use]
    pub const fn parse(raw: &[u8; STANDARD_LEN]) -> Self {
        let mut modes = [None; STANDARD_NUM];
        let mut i = 0;
        while i < STANDARD_NUM {
            let x_byte = raw[i * 2];
            let y_byte = raw[i * 2 + 1];
            modes[i] = parse_timing_bytes(x_byte, y_byte);
            i += 1;
        }
        Self { modes }
    }

    #[must_use]
    pub const fn mode(&self, i: usize) -> Option<Timing> {
        if i < STANDARD_NUM {
            self.modes[i]
        } else {
            None
        }
    }
}

#[must_use]
pub const fn parse_timing_bytes(x_byte: u8, y_byte: u8) -> Option<Timing> {
    if x_byte == 0b1 && y_byte == 0b1 {
        return None;
    }
    let width = (x_byte as u16 + 31) * 8;
    let aspect = match y_byte & 0b1100_0000 {
        0b0000_0000 => Aspect::A16_10,
        0b0100_0000 => Aspect::A4_3,
        0b1000_0000 => Aspect::A5_4,
        _ => Aspect::A16_9,
    };
    let height = match aspect {
        Aspect::A16_10 => width * 10 / 16,
        Aspect::A4_3 => width * 3 / 4,
        Aspect::A5_4 => width * 4 / 5,
        Aspect::A16_9 => width * 9 / 16,
    };
    let vfreq = (y_byte & 0b0011_1111) + 60;
    Some(Timing {
        width,
        height,
        aspect,
        vfreq,
    })
}
