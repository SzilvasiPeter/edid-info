//! Display Range Limits & Additional Timing descriptor (tag 0xFD).
//!
//! | Byte | Description |
//! |------|-------------|
//! | 4    | Offsets for rate over-255 extensions |
//! | 5    | Minimum vertical rate (Hz) |
//! | 6    | Maximum vertical rate (Hz) |
//! | 7    | Minimum horizontal rate (kHz) |
//! | 8    | Maximum horizontal rate (kHz) |
//! | 9    | Maximum pixel clock (×10 MHz) |
//! | 10   | Video timing support flag (00h=GTF, 01h=limits only, 02h=GTF secondary, 04h=CVT) |
//! | 11–17 | Timing-specific data (GTF secondary curve or CVT parameters) |
use crate::common::{AspectRatio, FailureKind, Validation, Version, WarningKind};

/// Video timing formula of the range descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoTimingSupport {
    /// Default GTF is supported (C=40, M=600, K=128, J=20).
    DefaultGtf,
    /// No additional timing formula; only range limits.
    RangeLimitsOnly,
    /// GTF secondary curve is supported.
    GtfSecondaryCurve(GtfSecondaryCurve),
    /// CVT (Coordinated Video Timing) is supported.
    Cvt(Cvt),
    /// Reserved value — do not use.
    Reserved(u8),
}

/// Display scaling capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scaling {
    None,
    Shrink,
    Stretch,
    Both,
}

/// Minimum and maximum rate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RateRange {
    pub min: u16,
    pub max: u16,
}

/// Parsed display range limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RangeLimits {
    offsets: u8,
    raw: [u8; 13],
}

impl RangeLimits {
    pub(super) const fn new(offsets: u8, raw: &[u8; 13]) -> Self {
        Self { offsets, raw: *raw }
    }

    /// Minimum and maximum vertical refresh rate in Hz.
    #[must_use]
    pub const fn vertical_hz(&self) -> RateRange {
        let (min, max) = adjust(self.raw[0], self.raw[1], self.offsets & 0b11);
        RateRange { min, max }
    }

    /// Minimum and maximum horizontal scan rate in kHz.
    #[must_use]
    pub const fn horizontal_khz(&self) -> RateRange {
        let (min, max) = adjust(self.raw[2], self.raw[3], (self.offsets >> 2) & 0b11);
        RateRange { min, max }
    }

    /// Maximum pixel clock in MHz.
    #[must_use]
    pub const fn max_pixel_clock_mhz(&self) -> u16 {
        (self.raw[4] as u16) * 10
    }

    /// Video timing formula type.
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
            0x02 => VideoTimingSupport::GtfSecondaryCurve(GtfSecondaryCurve { raw }),
            0x04 => VideoTimingSupport::Cvt(Cvt { raw }),
            v => VideoTimingSupport::Reserved(v),
        }
    }

    /// Validates the range limits descriptor against the VESA specification.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        let mut validation = Validation::new();
        let reserved_offset =
            matches!(self.offsets, 0x01 | 0x04..=0x07 | 0x09 | 0x0D | 0x10..=0xFF);
        let (v, h) = (self.vertical_hz(), self.horizontal_khz());
        validation = validation
            .fail_if(reserved_offset, FailureKind::RangeLimitsOffsetReserved)
            .fail_if(self.raw[0] == 0, FailureKind::RangeLimitsVerticalMinZero)
            .fail_if(self.raw[1] == 0, FailureKind::RangeLimitsVerticalMaxZero)
            .fail_if(self.raw[2] == 0, FailureKind::RangeLimitsHorizontalMinZero)
            .fail_if(self.raw[3] == 0, FailureKind::RangeLimitsHorizontalMaxZero)
            .fail_if(self.raw[4] == 0, FailureKind::RangeLimitsMaxPixelClockZero)
            .fail_if(
                v.min > v.max || h.min > h.max,
                FailureKind::RangeLimitsMinExceedsMax,
            );

        validation = validation
            .warn_if(
                matches!(self.timing(), VideoTimingSupport::Reserved(_)),
                WarningKind::VideoTimingSupportReserved,
            )
            .warn_if(
                matches!(self.raw[5], 0x00 | 0x02),
                WarningKind::GtfIsDeprecated,
            )
            .warn_if(
                matches!(self.raw[5], 0x00 | 0x01) && self.raw[6] != 0x0A,
                WarningKind::RangeLimitsExpectedLineFeed,
            )
            .warn_if(
                matches!(self.raw[5], 0x00 | 0x01)
                    && (self.raw[7] != 0x20
                        || self.raw[8] != 0x20
                        || self.raw[9] != 0x20
                        || self.raw[10] != 0x20
                        || self.raw[11] != 0x20
                        || self.raw[12] != 0x20),
                WarningKind::RangeLimitsExpectedSpaces,
            );

        validation = validation.then(match self.timing() {
            VideoTimingSupport::Cvt(cvt) => cvt.validate(),
            VideoTimingSupport::GtfSecondaryCurve(gtf) => gtf.validate(),
            _ => Validation::new(),
        });

        validation
    }
}

/// GTF secondary curve parameters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GtfSecondaryCurve {
    raw: [u8; 7],
}

impl GtfSecondaryCurve {
    /// Raw byte constructor.
    #[must_use]
    pub const fn new(raw: [u8; 7]) -> Self {
        Self { raw }
    }

    /// Start break frequency for the secondary curve in kHz.
    #[must_use]
    pub const fn start_khz(&self) -> u16 {
        (self.raw[1] as u16) * 2
    }

    /// Extended offset constant C (%).
    #[must_use]
    pub const fn c(&self) -> f32 {
        (self.raw[2] as f32) / 2.0
    }

    /// Extended gradient constant M (%/kHz).
    #[must_use]
    pub const fn m(&self) -> u16 {
        u16::from_le_bytes([self.raw[3], self.raw[4]])
    }

    /// Blanking time scaling factor K.
    #[must_use]
    pub const fn k(&self) -> u8 {
        self.raw[5]
    }

    /// Scaling factor weighting J (%).
    #[must_use]
    pub const fn j(&self) -> f32 {
        (self.raw[6] as f32) / 2.0
    }

    /// Validates GTF parameters against the VESA specification.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new().warn_if(self.raw[0] != 0, WarningKind::GtfSecondaryReservedByte)
    }
}

/// CVT blanking support flags.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Blanking {
    pub reduced: bool,
    pub standard: bool,
}

/// CVT timing parameters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cvt {
    raw: [u8; 7],
}

impl Cvt {
    /// Raw byte constructor.
    #[must_use]
    pub const fn new(raw: [u8; 7]) -> Self {
        Self { raw }
    }

    /// CVT standard version.
    #[must_use]
    pub const fn version(&self) -> Version {
        let major = (self.raw[0] >> 4) & 0x0F;
        let minor = self.raw[0] & 0x0F;
        Version { major, minor }
    }

    /// Additional pixel clock precision in kHz.
    #[must_use]
    pub const fn pixel_clock_precision_khz(&self) -> u16 {
        ((self.raw[1] >> 2) as u16) * 250
    }

    /// Maximum horizontal active pixels.
    #[must_use]
    pub const fn max_horizontal_active(&self) -> u16 {
        let msb = self.raw[1] & 0b11;
        let lsb = self.raw[2];
        u16::from_be_bytes([msb, lsb]) * 8
    }

    /// Supported aspect ratios.
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

    /// Preferred aspect ratio.
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

    /// CVT blanking support.
    #[must_use]
    pub const fn blanking(&self) -> Blanking {
        Blanking {
            reduced: (self.raw[4] & 0b0001_0000) != 0,
            standard: (self.raw[4] & 0b0000_1000) != 0,
        }
    }

    /// Horizontal scaling support.
    #[must_use]
    pub const fn horizontal_scaling(&self) -> Scaling {
        match (self.raw[5] >> 6) & 0b11 {
            0b01 => Scaling::Stretch,
            0b10 => Scaling::Shrink,
            0b11 => Scaling::Both,
            _ => Scaling::None,
        }
    }

    /// Vertical scaling support.
    #[must_use]
    pub const fn vertical_scaling(&self) -> Scaling {
        match (self.raw[5] >> 4) & 0b11 {
            0b01 => Scaling::Stretch,
            0b10 => Scaling::Shrink,
            0b11 => Scaling::Both,
            _ => Scaling::None,
        }
    }

    /// Preferred vertical refresh rate in Hz.
    #[must_use]
    pub const fn preferred_vertical_hz(&self) -> u8 {
        self.raw[6]
    }

    /// Validates CVT parameters against the VESA specification.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new()
            .fail_if(self.raw[0] & 0xF0 == 0, FailureKind::CvtVersionZero)
            .fail_if(self.raw[6] == 0, FailureKind::CvtPreferredRateZero)
            .warn_if(
                self.raw[3] & 0b0000_0111 != 0,
                WarningKind::CvtReservedAspectBits,
            )
            .warn_if(
                self.preferred_aspect().is_none(),
                WarningKind::CvtPreferredAspectReserved,
            )
            .warn_if(
                self.raw[4] & 0b0000_0111 != 0,
                WarningKind::CvtReservedBlankingBits,
            )
            .warn_if(
                self.raw[5] & 0b0000_1111 != 0,
                WarningKind::CvtReservedScalingBits,
            )
    }
}

const fn adjust(min: u8, max: u8, mode: u8) -> (u16, u16) {
    match mode {
        0b10 => (min as u16, max as u16 + 255),
        0b11 => (min as u16 + 255, max as u16 + 255),
        _ => (min as u16, max as u16),
    }
}
