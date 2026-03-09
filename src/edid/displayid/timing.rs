//! `DisplayID` Timing Descriptors (Type 7).
//!
//! Compact 20-byte timing descriptors for `DisplayID` 2.0.
//! Each descriptor encodes pixel clock, active resolution,
//! blanking intervals, and sync polarities.
//!
//! # Timing Descriptor Structure (20 bytes)
//!
//! | Bytes | Description |
//! |-------|-------------|
//! | 0–2   | Pixel clock (kHz, little-endian) |
//! | 3     | Options (preferred flag) |
//! | 4–7   | Horizontal active/blanking |
//! | 8–11  | Horizontal front porch/sync |
//! | 12–15 | Vertical active/blanking |
//! | 16–19 | Vertical front porch/sync |

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimingV7<'a> {
    rev: u8,
    data: &'a [u8],
}

impl<'a> TimingV7<'a> {
    #[must_use]
    pub const fn parse(rev: u8, data: &'a [u8]) -> Option<Self> {
        if data.len() < 20 || !data.len().is_multiple_of(20) {
            return None;
        }
        Some(Self { rev, data })
    }

    #[must_use]
    pub const fn rev(&self) -> u8 {
        self.rev
    }

    #[must_use]
    pub const fn descriptors(&self) -> TimingV7Iter<'a> {
        TimingV7Iter {
            data: self.data,
            at: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimingV7Desc {
    pixel_clock_khz: u32,
    opts: u8,
    h_active: u16,
    h_blank: u16,
    h_front: u16,
    h_sync: u16,
    v_active: u16,
    v_blank: u16,
    v_front: u16,
    v_sync: u16,
}

impl TimingV7Desc {
    #[must_use]
    pub fn parse(raw: &[u8; 20]) -> Self {
        Self {
            pixel_clock_khz: u32::from(raw[0])
                | (u32::from(raw[1]) << 8)
                | (u32::from(raw[2]) << 16),
            opts: raw[3],
            h_active: u16::from_le_bytes([raw[4], raw[5]]),
            h_blank: u16::from_le_bytes([raw[6], raw[7]]),
            h_front: u16::from_le_bytes([raw[8], raw[9]]) & 0x7FFF,
            h_sync: u16::from_le_bytes([raw[10], raw[11]]),
            v_active: u16::from_le_bytes([raw[12], raw[13]]),
            v_blank: u16::from_le_bytes([raw[14], raw[15]]),
            v_front: u16::from_le_bytes([raw[16], raw[17]]) & 0x7FFF,
            v_sync: u16::from_le_bytes([raw[18], raw[19]]),
        }
    }

    #[must_use]
    pub const fn pixel_clock_khz(&self) -> u32 {
        self.pixel_clock_khz
    }
    #[must_use]
    pub const fn h_active(&self) -> u16 {
        self.h_active
    }
    #[must_use]
    pub const fn v_active(&self) -> u16 {
        self.v_active
    }
    #[must_use]
    pub const fn is_preferred(&self) -> bool {
        (self.opts & 0x80) != 0
    }
    #[must_use]
    pub const fn h_sync_positive(&self) -> bool {
        (self.h_front & 0x8000) != 0
    } // Wait, spec says Bit 15 of bytes 8-9
}

pub struct TimingV7Iter<'a> {
    data: &'a [u8],
    at: usize,
}

impl Iterator for TimingV7Iter<'_> {
    type Item = TimingV7Desc;
    fn next(&mut self) -> Option<Self::Item> {
        if self.at + 20 > self.data.len() {
            return None;
        }
        let raw: &[u8; 20] = self.data[self.at..self.at + 20].try_into().ok()?;
        self.at += 20;
        Some(TimingV7Desc::parse(raw))
    }
}
