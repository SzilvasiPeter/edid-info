//! Detailed Timing Descriptor (DTD).
//!
//! An 18-byte structure describing a single video timing mode with
//! precise parameters including pixel clock, active pixels, blanking, sync polarities, and physical display size.
//!
//! # Descriptor Layout
//!
//! | Bytes | Description |
//! |-------|-------------|
//! | 0–1   | Pixel clock (×10 kHz, little-endian) |
//! | 2–4   | Horizontal active + blanking |
//! | 5–7   | Vertical active + blanking |
//! | 8–11  | Front porch, sync width (h/v) |
//! | 12–13 | Image size (mm, little-endian) |
//! | 14    | Border size |
//! | 15–17 | Flags (interlace, stereo, sync type) |
//!
//! # Examples
//!
//! ```rust
//! use edid_info::base::descriptor::dtd::{
//!     DetailedTiming, Stereo, SyncSignal,
//! };
//! use edid_info::common::{
//!     DESC_LEN, Polarity, Size, SyncPolarity, Timing,
//! };
//!
//! let mut raw = [0u8; DESC_LEN];
//! raw[0] = 0x01; // Pixel clock LSB (10 kHz × 1 = 10 kHz)
//! raw[2] = 100;  // h_active LSB
//! raw[3] = 20;   // h_blank LSB
//! raw[5] = 100;  // v_active LSB
//! raw[6] = 20;   // v_blank LSB
//! raw[8] = 5;    // h_front LSB
//! raw[9] = 3;    // h_sync LSB
//! raw[10] = 0x53; // v_front LSB [7:4]=5, v_sync LSB [3:0]=3
//! raw[12] = 44;  // physical width LSB
//! raw[13] = 30;  // physical height LSB
//! raw[17] = 0x1E; // DigitalSeparate(Positive, Positive), stereo=None
//!
//! let timing = DetailedTiming::new(&raw);
//! assert_eq!(timing.pixel_clock_khz(), 10);
//! assert!(!timing.interlaced());
//! assert_eq!(timing.horizontal(), Timing::new(100, 20, 5, 3, 0));
//! assert_eq!(timing.vertical(), Timing::new(100, 20, 5, 3, 0));
//! assert_eq!(timing.physical(), Size { width: 44, height: 30 });
//! assert_eq!(timing.stereo(), Stereo::None);
//! assert_eq!(
//!     timing.signal(),
//!     SyncSignal::DigitalSeparate(SyncPolarity {
//!         horizontal: Polarity::Positive,
//!         vertical: Polarity::Positive,
//!     })
//! );
//! assert!(timing.validate().is_valid());
//! ```

use crate::bit::{get_bits, is_set, pack_bits};
use crate::common::{
    DESC_LEN, FailureKind, Polarity, Size, SyncPolarity, Timing, Validation, WarningKind,
};

/// Stereo viewing support modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stereo {
    /// Normal display, no stereo.
    None,
    /// Field sequential, right image when sync = 1.
    FieldSeqRight,
    /// Field sequential, left image when sync = 1.
    FieldSeqLeft,
    /// 2-way interleaved, right image on even lines.
    TwoWayRightEven,
    /// 2-way interleaved, left image on even lines.
    TwoWayLeftEven,
    /// 4-way interleaved.
    FourWay,
    /// Side-by-side interleaved.
    SideBySide,
}

/// Sync signal interface definition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyncSignal {
    /// Analog composite sync signal.
    AnalogComposite {
        /// Uses bipolar sync signal.
        bipolar: bool,
        /// Includes V-sync serrations.
        serrations: bool,
        /// Signal sync source channel.
        source: AnalogSource,
    },
    /// Digital composite sync signal.
    DigitalComposite {
        /// Includes V-sync serrations.
        serrations: bool,
        /// Horizontal sync polarity.
        h_polarity: Polarity,
    },
    /// Digital separate sync signal with polarities.
    DigitalSeparate(SyncPolarity),
}

/// Sync source channel for analog composite.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalogSource {
    /// Sync on green channel only.
    GreenOnly,
    /// Sync on all RGB channels.
    Rgb,
}

/// Detailed Timing Descriptor (DTD) parser.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DetailedTiming {
    raw: [u8; DESC_LEN],
}

impl DetailedTiming {
    /// Creates a new detailed timing descriptor.
    #[must_use]
    pub const fn new(raw: &[u8; DESC_LEN]) -> Self {
        Self { raw: *raw }
    }

    /// Pixel clock frequency in kHz.
    #[must_use]
    pub const fn pixel_clock_khz(&self) -> u32 {
        u16::from_le_bytes([self.raw[0], self.raw[1]]) as u32 * 10
    }

    /// Horizontal scan timing parameters.
    #[must_use]
    pub const fn horizontal(&self) -> Timing {
        let h_active = pack_bits(get_bits(self.raw[4], 4, 7), self.raw[2], 8);
        let h_blank = pack_bits(get_bits(self.raw[4], 0, 3), self.raw[3], 8);
        let h_front = pack_bits(get_bits(self.raw[11], 6, 7), self.raw[8], 8);
        let h_sync = pack_bits(get_bits(self.raw[11], 4, 5), self.raw[9], 8);
        Timing::new(h_active, h_blank, h_front, h_sync, self.raw[15])
    }

    /// Vertical scan timing parameters.
    #[must_use]
    pub const fn vertical(&self) -> Timing {
        let raw = self.raw;
        let v_active = pack_bits(get_bits(raw[7], 4, 7), raw[5], 8);
        let v_blank = pack_bits(get_bits(raw[7], 0, 3), raw[6], 8);
        let v_front = pack_bits(get_bits(raw[11], 2, 3), get_bits(raw[10], 4, 7), 4);
        let v_sync = pack_bits(get_bits(raw[11], 0, 1), get_bits(raw[10], 0, 3), 4);
        Timing::new(v_active, v_blank, v_front, v_sync, raw[16])
    }

    /// Physical display dimensions in millimeters.
    #[must_use]
    pub const fn physical(&self) -> Size {
        let width_mm = pack_bits(get_bits(self.raw[14], 4, 7), self.raw[12], 8);
        let height_mm = pack_bits(get_bits(self.raw[14], 0, 3), self.raw[13], 8);
        Size {
            width: width_mm,
            height: height_mm,
        }
    }

    /// Whether the video interface is interlaced.
    #[must_use]
    pub const fn interlaced(&self) -> bool {
        is_set(self.raw[17], 7)
    }

    /// Stereo viewing support mode.
    #[must_use]
    pub const fn stereo(&self) -> Stereo {
        let raw = self.raw[17];
        match ((raw >> 5) & 0b11, raw & 0b0000_0001) {
            (0b01, 0) => Stereo::FieldSeqRight,
            (0b10, 0) => Stereo::FieldSeqLeft,
            (0b01, 1) => Stereo::TwoWayRightEven,
            (0b10, 1) => Stereo::TwoWayLeftEven,
            (0b11, 0) => Stereo::FourWay,
            (0b11, 1) => Stereo::SideBySide,
            _ => Stereo::None,
        }
    }

    /// Sync signal interface configuration.
    #[must_use]
    pub const fn signal(&self) -> SyncSignal {
        const fn polarity(bit: bool) -> Polarity {
            let pos = Polarity::Positive;
            let neg = Polarity::Negative;
            if bit { pos } else { neg }
        }

        let raw = self.raw[17];
        let b1 = is_set(raw, 1);
        let b2 = is_set(raw, 2);
        let rgb = AnalogSource::Rgb;
        let green = AnalogSource::GreenOnly;
        match (raw >> 3) & 0b11 {
            0b00 | 0b01 => SyncSignal::AnalogComposite {
                bipolar: is_set(raw, 3),
                serrations: b2,
                source: if b1 { rgb } else { green },
            },
            0b10 => SyncSignal::DigitalComposite {
                serrations: b2,
                h_polarity: polarity(b1),
            },
            _ => SyncSignal::DigitalSeparate(SyncPolarity {
                horizontal: polarity(b1),
                vertical: polarity(b2),
            }),
        }
    }

    /// Validates the detailed timing parameters.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        let width = self.physical().width;
        let height = self.physical().height;
        Validation::new()
            .fail_if(
                self.pixel_clock_khz() == 0,
                FailureKind::TimingPixelClockIsZero,
            )
            // Warn if only one dimension is zero. Both dimensions being zero is a
            // valid configuration for projectors with undefined screen sizes.
            .warn_if((width == 0) != (height == 0), WarningKind::DubiousImageSize)
    }
}
