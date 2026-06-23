//! Common helpers for EDID parsing.

/// Length of an EDID block (base or extension) in bytes.
pub const BLOCK_LEN: usize = 128;

/// Length of an descriptor (detailed timing or monitor) in bytes.
pub const DESC_LEN: usize = 18;

/// Error variants for EDID validation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FailureKind {
    /// Invalid checksum for the base block.
    BaseChecksumMismatch = 0,
    /// Specified extension count differs from actual.
    ExtensionCountMismatch = 1,
    /// Manufacturer ID contains invalid bits.
    ManufacturerIdIsInvalid = 2,
    /// Manufacture week value is invalid.
    ManufactureWeekIsInvalid = 3,
    /// Major version is zero.
    MajorVersionIsZero = 4,
    /// Color bit depth set to reserved value.
    ColorBitDepthIsReserved = 5,
    /// Digital video interface set to reserved value.
    VideoInterfaceIsReserved = 6,
    /// sRGB is signaled, but chromaticities do not match.
    StandardRgbChromaMismatch = 7,
    /// Standard timing horizontal pixels are outside 256–2288.
    StdTimingHorizontalLimit = 8,
    /// Standard timing refresh rate exceeds 123 Hz.
    StdTimingRefreshLimit = 9,
    /// Use 0x01 0x01 byte code for empty Standard Timings.
    InvalidEmptyStdTiming = 10,
    /// The first descriptor is not a detailed timing descriptor.
    FirstDescriptorNotDetailedTiming = 11,
    /// Timing descriptors shall precede display descriptors.
    InvalidDescriptorOrder = 12,
    /// Monitor descriptor reserved byte is non-zero.
    MonitorReservedByteIsNonZero = 13,
    /// Descriptor is all zeroes, use Dummy Descriptor.
    AllZeroDescriptor = 14,
    /// Detailed timing descriptor Pixel clock is zero.
    TimingPixelClockIsZero = 15,
    /// Undefined display descriptor tag.
    UndefinedDescriptor = 16,
    /// Range limits minimum vertical rate is zero.
    RangeLimitsVerticalMinZero = 17,
    /// Range limits maximum vertical rate is zero.
    RangeLimitsVerticalMaxZero = 18,
    /// Range limits minimum horizontal rate is zero.
    RangeLimitsHorizontalMinZero = 19,
    /// Range limits maximum horizontal rate is zero.
    RangeLimitsHorizontalMaxZero = 20,
    /// Range limits maximum pixel clock is zero.
    RangeLimitsMaxPixelClockZero = 21,
    /// Range limits minimum exceeds maximum.
    RangeLimitsMinExceedsMax = 22,
    /// GTF and CVT requires continuous frequency display.
    GtfAndCvtRequiresContFreq = 23,
    /// Range limits offset byte contains a reserved value.
    RangeLimitsOffsetReserved = 24,
    /// CVT standard version is zero (major nibble is 0).
    CvtVersionZero = 25,
    /// CVT preferred vertical refresh rate is zero (reserved).
    CvtPreferredRateZero = 26,
}

impl FailureKind {
    /// Returns the human-readable error message.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::BaseChecksumMismatch => "Invalid base checksum",
            Self::ExtensionCountMismatch => "Specified extension count differs from actual",
            Self::ManufacturerIdIsInvalid => "Manufacturer ID contains invalid bits",
            Self::ManufactureWeekIsInvalid => "Manufacture week value is invalid",
            Self::MajorVersionIsZero => "Major version is zero",
            Self::ColorBitDepthIsReserved => "Color bit depth set to reserved value",
            Self::VideoInterfaceIsReserved => "Digital video interface set to reserved value",
            Self::StandardRgbChromaMismatch => "sRGB is signaled, but chromaticities do not match",
            Self::InvalidDescriptorOrder => "Timing descriptors shall precede display descriptors",
            Self::MonitorReservedByteIsNonZero => "Monitor descriptor reserved byte must be zero",
            Self::AllZeroDescriptor => "Descriptor is all zeroes, use Dummy Descriptor",
            Self::TimingPixelClockIsZero => "Detailed timing descriptor Pixel clock is zero",
            Self::FirstDescriptorNotDetailedTiming => "First descriptor is not a detailed timing",
            Self::StdTimingHorizontalLimit => "Standard timing horizontal pixels outside 256-2288",
            Self::StdTimingRefreshLimit => "Standard timing refresh rate exceeds 123 Hz",
            Self::InvalidEmptyStdTiming => "Use 0x01 0x01 byte code for empty Standard Timings",
            Self::UndefinedDescriptor => "Undefined display descriptor tag",
            Self::RangeLimitsVerticalMinZero => "Range limits minimum vertical rate is zero",
            Self::RangeLimitsVerticalMaxZero => "Range limits maximum vertical rate is zero",
            Self::RangeLimitsHorizontalMinZero => "Range limits minimum horizontal rate is zero",
            Self::RangeLimitsHorizontalMaxZero => "Range limits maximum horizontal rate is zero",
            Self::RangeLimitsMaxPixelClockZero => "Range limits maximum pixel clock is zero",
            Self::RangeLimitsMinExceedsMax => "Range limits minimum exceeds maximum",
            Self::GtfAndCvtRequiresContFreq => "GTF and CVT requires continous frequency display",
            Self::RangeLimitsOffsetReserved => "Range limits offset byte contains reserved value",
            Self::CvtVersionZero => "CVT standard version major nibble is zero",
            Self::CvtPreferredRateZero => "CVT preferred vertical refresh rate is zero (reserved)",
        }
    }
}

/// Warning variants for EDID validation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WarningKind {
    /// Manufacturer ID reserved bit is set.
    ManufacturerIdReservedIsSet = 0,
    /// Product code is zero.
    ProductCodeIsZero = 1,
    /// Serial number is zero.
    SerialNumberIsZero = 2,
    /// Version is deprecated, use 1.4 or later.
    VersionIsDeprecated = 3,
    /// Standard Timing has a dubious odd vertical resolution.
    StdTimingOddVertical = 4,
    /// Dubious image size (zero or suspiciously small).
    DubiousImageSize = 5,
    /// sRGB is signaled, but the gamma is not 2.2.
    StandardRgbGammaMismatch = 6,
    /// Chromaticities match sRGB, but sRGB is not signaled.
    StandardRgbNotSignaled = 7,
    /// Monochrome display has non-zero RGB chromaticities.
    MonochromeHasNonZeroRgb = 8,
    /// Monitor name is missing.
    MissingMonitorName = 9,
    /// Range limits required for continuous frequency.
    RangeLimitsRequired = 10,
    /// GTF is deprecated, use CVT timing formula.
    GtfIsDeprecated = 11,
    /// Video timing support contains a reserved value.
    VideoTimingSupportReserved = 12,
    /// Expected line feed for Default GTF or Range Limits Only.
    RangeLimitsExpectedLineFeed = 13,
    /// Expected spaces for Default GTF or Range Limits Only.
    RangeLimitsExpectedSpaces = 14,
    /// CVT supported aspect ratios reserved bits are non-zero.
    CvtReservedAspectBits = 15,
    /// CVT blanking reserved bits are non-zero.
    CvtReservedBlankingBits = 16,
    /// CVT scaling reserved bits are non-zero.
    CvtReservedScalingBits = 17,
    /// CVT preferred aspect ratio is a reserved value.
    CvtPreferredAspectReserved = 18,
    /// GTF secondary curve reserved byte is non-zero.
    GtfSecondaryReservedByte = 19,
}

impl WarningKind {
    /// Returns the human-readable warning message.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::ManufacturerIdReservedIsSet => "Manufacturer ID reserved bit is set",
            Self::ProductCodeIsZero => "Product code is zero",
            Self::SerialNumberIsZero => "Serial number is zero",
            Self::VersionIsDeprecated => "Version is deprecated, use 1.4 or later",
            Self::StdTimingOddVertical => "Standard Timing has a dubious odd vertical resolution",
            Self::DubiousImageSize => "Dubious image size (zero or suspiciously small)",
            Self::StandardRgbGammaMismatch => "sRGB signaled, but gamma is not 2.2",
            Self::StandardRgbNotSignaled => "Chromaticities match sRGB, but sRGB not signaled",
            Self::MonochromeHasNonZeroRgb => "Monochrome display has non-zero RGB chromaticities",
            Self::MissingMonitorName => "Monitor name is missing",
            Self::RangeLimitsRequired => "Range limits required for continous frequency",
            Self::GtfIsDeprecated => "GTF is deprecated, use CVT instead",
            Self::VideoTimingSupportReserved => "Video timing support is set to a reserved value",
            Self::RangeLimitsExpectedLineFeed => "Expected line feed for range limits/default GTF",
            Self::RangeLimitsExpectedSpaces => "Expected spaces for range limits/default GTF",
            Self::CvtReservedAspectBits => "CVT supported aspect ratios reserved bits are non-zero",
            Self::CvtReservedBlankingBits => "CVT blanking reserved bits are non-zero",
            Self::CvtReservedScalingBits => "CVT scaling reserved bits are non-zero",
            Self::CvtPreferredAspectReserved => "CVT preferred aspect ratio is a reserved value",
            Self::GtfSecondaryReservedByte => "GTF secondary curve reserved byte is non-zero",
        }
    }
}

/// Validation result with errors and warnings represented as bitfields.
/// Error and warning enums must each fit within 64 variants.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Validation {
    /// Fatal errors bitfield.
    pub errors: u64,
    /// Non-fatal warnings bitfield.
    pub warnings: u64,
}

impl Validation {
    /// Create a new empty validation result.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            errors: 0,
            warnings: 0,
        }
    }

    /// Check if validation passed (no errors).
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.errors == 0
    }

    /// Merge another validation result into this one.
    #[must_use]
    pub const fn then(self, other: Self) -> Self {
        Self {
            errors: self.errors | other.errors,
            warnings: self.warnings | other.warnings,
        }
    }

    /// Add a failure if condition is true.
    #[must_use]
    pub const fn fail_if(mut self, cond: bool, kind: FailureKind) -> Self {
        if cond {
            self.errors |= 1 << (kind as u8);
        }
        self
    }

    /// Add a warning if condition is true.
    #[must_use]
    pub const fn warn_if(mut self, cond: bool, kind: WarningKind) -> Self {
        if cond {
            self.warnings |= 1 << (kind as u8);
        }
        self
    }
}

/// Polarity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Polarity {
    Positive,
    Negative,
}

/// Synchronization polarity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyncPolarity {
    pub horizontal: Polarity,
    pub vertical: Polarity,
}

/// EDID version (major and minor).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Physical dimensions in millimeters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

/// Aspect ratio represented as a width:height ratio.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AspectRatio {
    width: u16,
    height: u16,
}

impl AspectRatio {
    /// Creates a new aspect ratio.
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Creates an aspect ratio from physical dimensions.
    #[must_use]
    pub const fn from_size(size: Size) -> Self {
        if size.width == 0 && size.height == 0 {
            return Self {
                width: 0,
                height: 0,
            };
        }

        let common = gcd(size.width, size.height);
        Self {
            width: size.width / common,
            height: size.height / common,
        }
    }

    /// Width component of the ratio.
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    /// Height component of the ratio.
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }
}

/// Scan timing parameters for one axis.
///
/// A **line** (horizontal axis) is one row of pixels scanned left to right.
/// A **frame** (vertical axis) is one full screen of lines scanned top to bottom.
/// The GPU and monitor use these values to coordinate when pixels are sent
/// and when to pause for synchronization.
///
/// # Scan Cycle
///
/// ```text
/// ------------------------------------------------------
/// |  visible   | front porch | sync pulse | back porch |
/// | (user sees)|  (settle)   | (trigger)  | (stabilize)|
/// |---active---|--------- blanking interval -----------|
/// ```
///
/// The blanking interval exists because the GPU and monitor need time to
/// synchronize between passes:
///
/// - **Front porch**: the receiver settles after active data ends,
///   ensuring the sync pulse is not mistaken for noise.
/// - **Sync pulse**: the retrace command — both sides reset their position
///   to the start of the next line or frame.
/// - **Back porch**: the sender and receiver stabilize after retrace
///   before visible data begins.
///
/// # Example: 1920×1080 @ 60 Hz (pixel clock 148.50 MHz)
///
/// Horizontal (per line): 1920 active pixels, 280 pixel-times of blanking.
/// The GPU renders 1920 pixels, then pauses for 280 pixel-times before the next line.
///
/// Vertical (per frame): 1080 active lines, 45 line-times of blanking.
/// After 1080 lines, the GPU pauses for 45 line-times before the next frame.
///
/// The refresh rate emerges from the pixel clock and total scan periods:
///
/// ```text
/// Hz = pixel_clock / (horizontal.total() × vertical.total())
///    = 148_500_000 / (2200 × 1125)
///    = 60.00 Hz
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timing {
    active: u16,
    blank: u16,
    front: u16,
    sync: u16,
    border: u8,
}

impl Timing {
    /// Creates scan timing parameters for one axis.
    #[must_use]
    pub const fn new(active: u16, blank: u16, front: u16, sync: u16, border: u8) -> Self {
        Self {
            active,
            blank,
            front,
            sync,
            border,
        }
    }

    /// Rendered horizontal or vertical pixels.
    #[must_use]
    pub const fn active(&self) -> u16 {
        self.active
    }

    /// Pause duration in pixel-times (line) or line-times (frame).
    #[must_use]
    pub const fn blank(&self) -> u16 {
        self.blank
    }

    /// Front porch duration.
    #[must_use]
    pub const fn front(&self) -> u16 {
        self.front
    }

    /// Sync pulse width.
    #[must_use]
    pub const fn sync(&self) -> u16 {
        self.sync
    }

    /// Back porch duration. This period helps stabilize the signal before the next active pixels are displayed.
    /// Formula: `back_porch = blank_time - (front_porch + sync_pulse)`
    #[must_use]
    pub const fn back(&self) -> u16 {
        self.blank
            .saturating_sub(self.front)
            .saturating_sub(self.sync)
    }

    /// Total scan period: active + blank. Used with the pixel clock to derive the refresh rate.
    #[must_use]
    pub const fn total(&self) -> u16 {
        self.active + self.blank
    }

    /// Border pixels/lines on one edge. Applied to both sides (left+right for horizontal, top+bottom for vertical),
    /// so the total border is twice this value. Typically zero.
    #[must_use]
    pub const fn border(&self) -> u8 {
        self.border
    }
}

/// Verifies the checksum of an EDID block.
///
/// Returns `true` if the sum of all bytes in the block equals zero (with u8 wrapping).
pub(crate) const fn checksum_ok(raw: &[u8; BLOCK_LEN]) -> bool {
    let mut sum = 0u8;
    let mut i = 0;
    while i < BLOCK_LEN {
        sum = sum.wrapping_add(raw[i]);
        i += 1;
    }
    sum == 0
}

const fn gcd(mut a: u16, mut b: u16) -> u16 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
