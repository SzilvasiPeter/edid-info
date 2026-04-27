//! Common helpers for EDID parsing.

/// Length of an EDID block (base or extension) in bytes.
pub const BLOCK_LEN: usize = 128;

/// Length of an descriptor (detailed timing or monitor) in bytes.
pub const DESC_LEN: usize = 18;

/// Error variants for EDID validation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FailureKind {
    // TODO: Remove the module prefix after including the warnings in the validation steps
    // --- Base & Footer ---
    /// Invalid checksum for the base block.
    BaseChecksum = 0,
    /// Extension count in footer does not match parsed blocks.
    BaseExtCountMismatch = 1,

    // --- Header ---
    /// Manufacturer ID contains invalid bits.
    HeaderMfrInvalidBits = 4,
    /// Week value is invalid (greater than 54 and not 0xFF).
    HeaderWeekInvalid = 5,
    /// Major version is zero.
    HeaderMajorInvalid = 6,

    // --- Basic Parameters ---
    /// Color Bit Depth set to reserved value.
    BasicColorDepthReserved = 7,
    /// Digital Video Interface Standard set to reserved value.
    BasicInterfaceReserved = 8,
    /// sRGB is signaled, but chromaticities do not match.
    BasicSrgbChromaMismatch = 32,
    /// Chromaticities match sRGB, but sRGB is not signaled.
    BasicSrgbNotSignaled = 33,

    // --- Descriptors ---
    /// Missing Display Product Name (mandatory in 1.4).
    DescriptorMissingDisplayName = 2,
    /// Missing Display Range Limits Descriptor (mandatory in 1.4).
    DescriptorMissingRangeLimits = 3,
    /// Invalid detailed timing descriptor ordering.
    DescriptorOrdering = 9,
    /// Monitor descriptor block has byte 2 nonzero.
    DescriptorMonitorByte2NonZero = 10,
    /// Monitor descriptor block has byte 4 nonzero.
    DescriptorMonitorByte4NonZero = 11,
    /// Descriptor is all zeroes (Dummy Descriptor should be used).
    DescriptorAllZeroes = 12,

    // --- Detailed Timing (DTD) ---
    /// Pixel clock is zero.
    TimingPixelClockIsZero = 13,
    /// Missing preferred timing.
    TimingMissingPreferred = 14,

    // --- Range Limits ---
    /// GTF is supported, but continuous frequencies are not.
    RangeGtfNotContinuous = 15,
    /// CVT is supported, but continuous frequencies are not.
    RangeCvtNotContinuous = 16,
    /// Range limits descriptor missing max dotclock.
    RangeMaxClockNotSet = 17,
    /// CVT descriptor byte 14 reserved bits are non-zero.
    RangeCvtReservedByte14 = 18,
    /// CVT descriptor invalid preferred aspect ratio.
    RangeCvtPrefAspectRatioInvalid = 19,
    /// CVT descriptor byte 15 reserved bits are non-zero.
    RangeCvtReservedByte15 = 20,
    /// CVT descriptor byte 16 reserved bits are non-zero.
    RangeCvtReservedByte16 = 21,

    // --- CVT 3-Byte ---
    /// CVT 3-byte timing descriptor has invalid version.
    Cvt3ByteInvalidVersion = 22,
    /// CVT 3-byte timing byte 0 is zero (reserved).
    Cvt3ByteByte0Zero = 23,
    /// CVT 3-byte timing byte 1 reserved bits are non-zero.
    Cvt3ByteByte1Reserved = 24,
    /// CVT 3-byte timing byte 2 reserved bit is non-zero.
    Cvt3ByteByte2Reserved = 25,
    /// CVT 3-byte timing byte 2 supports no vertical rates.
    Cvt3ByteNoVerticalRates = 26,
    /// CVT 3-byte timing preferred rate not supported.
    Cvt3BytePrefRateNotSupported = 27,

    // --- Standard Timing 3 ---
    /// Standard Timing 3 descriptor has invalid version.
    Std3ByteInvalidVersion = 28,
    /// Standard Timing 3 descriptor has non-zero trailing bytes.
    Std3ByteNonZeroTrailing = 29,

    // --- DCM ---
    /// DCM descriptor has invalid version.
    DcmInvalidVersion = 30,

    // --- Extension CTA ---
    /// Invalid checksum for a CTA extension block.
    CtaChecksum = 31,
}

impl FailureKind {
    /// Returns the human-readable error message.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::BaseChecksum => "Invalid base checksum",
            Self::BaseExtCountMismatch => "Extension count does not match parsed blocks",
            Self::HeaderMfrInvalidBits => "Invalid manufacturer ID bits",
            Self::HeaderWeekInvalid => "Invalid week value",
            Self::HeaderMajorInvalid => "Invalid major version",
            Self::BasicColorDepthReserved => "Color bit depth is set to a reserved value",
            Self::BasicInterfaceReserved => "Digital video interface is set to a reserved value",
            Self::BasicSrgbChromaMismatch => "sRGB signaled, but chromaticities do not match",
            Self::BasicSrgbNotSignaled => "Chromaticities match sRGB, but sRGB not signaled",
            Self::DescriptorMissingDisplayName => "Missing Display Product Name",
            Self::DescriptorMissingRangeLimits => "Missing Display Range Limits Descriptor",
            Self::DescriptorOrdering => "Invalid descriptor ordering",
            Self::DescriptorMonitorByte2NonZero => "Monitor descriptor has non-zero byte 2",
            Self::DescriptorMonitorByte4NonZero => "Monitor descriptor has non-zero byte 4",
            Self::DescriptorAllZeroes => "Descriptor is all zeroes (use dummy instead)",
            Self::TimingPixelClockIsZero => "Pixel clock is zero",
            Self::TimingMissingPreferred => "Missing preferred timing",
            Self::RangeGtfNotContinuous => "GTF supported but continuous frequencies not supported",
            Self::RangeCvtNotContinuous => "CVT supported but continuous frequencies not supported",
            Self::RangeMaxClockNotSet => "Range limits missing max dotclock",
            Self::RangeCvtReservedByte14 => "CVT descriptor byte 14 reserved bits are non-zero",
            Self::RangeCvtPrefAspectRatioInvalid => "CVT descriptor invalid preferred aspect ratio",
            Self::RangeCvtReservedByte15 => "CVT descriptor byte 15 reserved bits are non-zero",
            Self::RangeCvtReservedByte16 => "CVT descriptor byte 16 reserved bits are non-zero",
            Self::Cvt3ByteInvalidVersion => "CVT 3-byte timing descriptor has invalid version",
            Self::Cvt3ByteByte0Zero => "CVT 3-byte timing byte 0 is zero (reserved)",
            Self::Cvt3ByteByte1Reserved => "CVT 3-byte timing byte 1 reserved bits are non-zero",
            Self::Cvt3ByteByte2Reserved => "CVT 3-byte timing byte 2 reserved bit is non-zero",
            Self::Cvt3ByteNoVerticalRates => "CVT 3-byte timing byte 2 supports no vertical rates",
            Self::Cvt3BytePrefRateNotSupported => "CVT 3-byte timing preferred rate not supported",
            Self::Std3ByteInvalidVersion => "Standard Timing 3 has invalid version",
            Self::Std3ByteNonZeroTrailing => "Standard Timing 3 has non-zero trailing bytes",
            Self::DcmInvalidVersion => "DCM descriptor has invalid version",
            Self::CtaChecksum => "Invalid CTA checksum",
        }
    }
}

/// Warning variants for EDID validation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WarningKind {
    // TODO: Remove the module prefix after including the warnings in the validation steps
    // --- Header ---
    /// Manufacturer ID reserved bit (bit 15) is set.
    HeaderMfrReservedSet = 0,
    /// Product code is zero.
    HeaderProductInvalid = 1,
    /// Serial number is zero.
    HeaderSerialInvalid = 2,
    /// Version is deprecated.
    HeaderVersionDeprecated = 3,

    // --- Standard Timing ---
    /// Standard Timing has a dubious odd vertical resolution.
    StandardTimingOddVertical = 4,

    // --- Range Limits ---
    /// GTF support is deprecated.
    RangeGtfDeprecated = 5,
    /// CVT block corrects dotclock by more than 9.75 MHz.
    RangeCvtDotClockLargeCorrection = 6,
    /// CVT block does not set preferred refresh rate.
    RangeCvtNoPreferredRefresh = 7,

    // --- Basic Parameters ---
    /// Dubious image size (smaller than 10x10 cm).
    BasicImageSizeDubious = 8,
    /// sRGB is signaled, but the gamma is not 2.2.
    BasicSrgbGammaInvalid = 9,
    /// Chromaticities match sRGB, but sRGB is not signaled.
    BasicSrgbNotSignaled = 10,
    // --- Chromaticity ---
    /// Monochrome display has non-zero RGB chromaticities.
    ChromaMonochromeRgbNonZero = 11,
}

impl WarningKind {
    /// Returns the human-readable warning message.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::HeaderMfrReservedSet => "Manufacturer ID reserved bit is set",
            Self::HeaderProductInvalid => "Invalid product code",
            Self::HeaderSerialInvalid => "Invalid serial number",
            Self::HeaderVersionDeprecated => "Deprecated version",
            Self::StandardTimingOddVertical => "Standard timing has odd vertical resolution",
            Self::RangeGtfDeprecated => "GTF support is deprecated",
            Self::RangeCvtDotClockLargeCorrection => "CVT dotclock correction exceeds 9.75 MHz",
            Self::RangeCvtNoPreferredRefresh => "CVT block missing preferred refresh rate",
            Self::BasicImageSizeDubious => "Dubious image size (zero or suspiciously small)",
            Self::BasicSrgbGammaInvalid => "sRGB signaled, but gamma is not 2.2",
            Self::BasicSrgbNotSignaled => "Chromaticities match sRGB, but sRGB not signaled",
            Self::ChromaMonochromeRgbNonZero => {
                "Monochrome display has non-zero RGB chromaticities"
            }
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
    major: u8,
    minor: u8,
}

impl Version {
    /// Initialize a version from major and minor version.
    #[must_use]
    pub const fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Physical dimensions in millimeters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    width: u16,
    height: u16,
}

impl Size {
    /// Creates a new size in millimeters.
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Returns the width in millimeters.
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height in millimeters.
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }
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

/// Copy `N` bytes from `raw` starting at `off`.
pub(crate) fn slice<const N: usize, const M: usize>(raw: &[u8; M], off: usize) -> [u8; N] {
    let mut out = [0u8; N];
    out.copy_from_slice(&raw[off..off + N]);
    out
}

const fn gcd(mut a: u16, mut b: u16) -> u16 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
