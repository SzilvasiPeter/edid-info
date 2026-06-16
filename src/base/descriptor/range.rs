//! Specifies the supported vertical and horizontal frequency ranges, maximum pixel clock, and timing formula type (GTF, CVT).
//!
//! # Range Descriptor Structure (tag 0xFD)
//!
//! | Byte | Description |
//! |------|-------------|
//! | 5    | Minimum vertical rate (Hz) |
//! | 6    | Maximum vertical rate (Hz) |
//! | 7    | Minimum horizontal rate (kHz) |
//! | 8    | Maximum horizontal rate (kHz) |
//! | 9    | Maximum pixel clock (×10 MHz) |
use crate::common::{AspectRatio, FailureKind, Validation, Version, WarningKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoTimingSupport {
    DefaultGtf,
    RangeLimitsOnly,
    SecondaryGtf(GtfSecondaryCurve),
    Cvt(CvtSupport),
    Reserved(u8),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scaling {
    None,
    Shrink,
    Stretch,
    Both,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RangeLimits {
    offsets: u8,
    raw: [u8; 13],
}

impl RangeLimits {
    pub(super) const fn new(offsets: u8, raw: &[u8; 13]) -> Self {
        Self { offsets, raw: *raw }
    }

    #[must_use]
    pub const fn v_min_hz(&self) -> u16 {
        adjust(self.raw[0], self.raw[1], self.offsets & 0b11).0
    }

    #[must_use]
    pub const fn v_max_hz(&self) -> u16 {
        adjust(self.raw[0], self.raw[1], self.offsets & 0b11).1
    }

    #[must_use]
    pub const fn h_min_khz(&self) -> u16 {
        adjust(self.raw[2], self.raw[3], (self.offsets >> 2) & 0b11).0
    }

    #[must_use]
    pub const fn h_max_khz(&self) -> u16 {
        adjust(self.raw[2], self.raw[3], (self.offsets >> 2) & 0b11).1
    }

    #[must_use]
    pub const fn pixel_mhz(&self) -> u16 {
        (self.raw[4] as u16) * 10
    }

    #[must_use]
    pub const fn timing(&self) -> VideoTimingSupport {
        let raw = [
            self.raw[6],
            self.raw[7],
            self.raw[8],
            self.raw[9],
            self.raw[10],
            self.raw[11],
            self.raw[12],
        ];
        match self.raw[5] {
            0x00 => VideoTimingSupport::DefaultGtf,
            0x01 => VideoTimingSupport::RangeLimitsOnly,
            0x02 => VideoTimingSupport::SecondaryGtf(GtfSecondaryCurve { raw }),
            0x04 => VideoTimingSupport::Cvt(CvtSupport { raw }),
            v => VideoTimingSupport::Reserved(v),
        }
    }

    /// Validates the range limits descriptor against the VESA specification.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        let mut validation = Validation::new();
        // TODO: Use more specific failure kind on the reserved byte failures for better error diagnositc.
        validation = validation.fail_if(
            matches!(
                self.offsets,
                0x01 | 0x04..=0x07 | 0x09 | 0x0D | 0x10..=0xFF
            ),
            FailureKind::RangeLimitsReservedByte,
        );
        validation = validation.fail_if(
            self.raw[0] == 0
                || self.raw[1] == 0
                || self.raw[2] == 0
                || self.raw[3] == 0
                || self.raw[4] == 0,
            FailureKind::RangeLimitsReservedByte,
        );

        let (v_min, v_max) = adjust(self.raw[0], self.raw[1], self.offsets & 0b11);
        let (h_min, h_max) = adjust(self.raw[2], self.raw[3], (self.offsets >> 2) & 0b11);
        validation = validation.fail_if(
            v_min > v_max || h_min > h_max,
            FailureKind::RangeLimitsMinExceedsMax,
        );

        validation = validation.warn_if(
            matches!(self.timing(), VideoTimingSupport::Reserved(_)),
            WarningKind::VideoTimingSupportReserved,
        );
        validation = validation.warn_if(
            matches!(self.raw[5], 0x00 | 0x02),
            WarningKind::GtfIsDeprecated,
        );

        validation
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GtfSecondaryCurve {
    raw: [u8; 7],
}

impl GtfSecondaryCurve {
    #[must_use]
    pub const fn new(raw: [u8; 7]) -> Self {
        Self { raw }
    }

    #[must_use]
    pub const fn start_khz(&self) -> u16 {
        (self.raw[1] as u16) * 2
    }

    #[must_use]
    pub const fn c_x2(&self) -> u8 {
        self.raw[2]
    }

    #[must_use]
    pub const fn m(&self) -> u16 {
        u16::from_le_bytes([self.raw[3], self.raw[4]])
    }

    #[must_use]
    pub const fn k(&self) -> u8 {
        self.raw[5]
    }

    #[must_use]
    pub const fn j_x2(&self) -> u8 {
        self.raw[6]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Blanking {
    pub reduced: bool,
    pub standard: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CvtSupport {
    raw: [u8; 7],
}

impl CvtSupport {
    #[must_use]
    pub const fn new(raw: [u8; 7]) -> Self {
        Self { raw }
    }

    #[must_use]
    pub const fn version(&self) -> Version {
        let major = (self.raw[0] >> 4) & 0x0F;
        let minor = self.raw[0] & 0x0F;
        Version { major, minor }
    }

    #[must_use]
    pub const fn add_clock_0_25_mhz(&self) -> u8 {
        self.raw[1] >> 2
    }

    #[must_use]
    pub const fn max_active(&self) -> u16 {
        let msb = self.raw[1] & 0b11;
        let lsb = self.raw[2];
        u16::from_be_bytes([msb, lsb]) * 8
    }

    pub fn aspect_ratios(&self) -> impl Iterator<Item = AspectRatio> {
        let bits = self.raw[3];
        [
            (0b1000_0000, AspectRatio::new(4, 3)),
            (0b0100_0000, AspectRatio::new(16, 9)),
            (0b0010_0000, AspectRatio::new(16, 10)),
            (0b0001_0000, AspectRatio::new(5, 4)),
            (0b0000_1000, AspectRatio::new(15, 9)),
        ]
        .into_iter()
        .filter_map(move |(mask, ar)| (bits & mask != 0).then_some(ar))
    }

    #[must_use]
    pub const fn preferred_aspect(&self) -> Option<AspectRatio> {
        match (self.raw[4] >> 5) & 0b111 {
            0b000 => Some(AspectRatio::new(4, 3)),
            0b001 => Some(AspectRatio::new(16, 9)),
            0b010 => Some(AspectRatio::new(16, 10)),
            0b011 => Some(AspectRatio::new(5, 4)),
            0b100 => Some(AspectRatio::new(15, 9)),
            _ => None,
        }
    }

    #[must_use]
    pub const fn blanking(&self) -> Blanking {
        Blanking {
            reduced: (self.raw[4] & 0b0001_0000) != 0,
            standard: (self.raw[4] & 0b0000_1000) != 0,
        }
    }

    #[must_use]
    pub const fn horizontal_scaling(&self) -> Scaling {
        match (self.raw[5] >> 6) & 0b11 {
            0b01 => Scaling::Stretch,
            0b10 => Scaling::Shrink,
            0b11 => Scaling::Both,
            _ => Scaling::None,
        }
    }

    #[must_use]
    pub const fn vertical_scaling(&self) -> Scaling {
        match (self.raw[5] >> 4) & 0b11 {
            0b01 => Scaling::Stretch,
            0b10 => Scaling::Shrink,
            0b11 => Scaling::Both,
            _ => Scaling::None,
        }
    }

    #[must_use]
    pub const fn pref_v_hz(&self) -> u8 {
        self.raw[6]
    }
}

const fn adjust(min: u8, max: u8, mode: u8) -> (u16, u16) {
    match mode {
        0b10 => (min as u16, max as u16 + 255),
        0b11 => (min as u16 + 255, max as u16 + 255),
        _ => (min as u16, max as u16),
    }
}
