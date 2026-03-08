/// Represents a display aspect ratio.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AspectRatio {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vic {
    name: &'static str,
    dar: AspectRatio,
    par: AspectRatio,
    pixel_clock_mhz: u32,
    vfreq_hz: u32,
    hfreq_hz: u32,
    width: u16,
    height: u16,
    total_h: u16,
    total_v: u16,
    field_rate_hz: u32,
}

#[rustfmt::skip]
impl Vic {
    #[must_use]
    pub const fn name(&self) -> &'static str { self.name }
    #[must_use]
    pub const fn dar(&self) -> AspectRatio { self.dar }
    #[must_use]
    pub const fn par(&self) -> AspectRatio { self.par }
    #[must_use]
    pub const fn pixel_clock_mhz(&self) -> u32 { self.pixel_clock_mhz }
    #[must_use]
    pub const fn vfreq_hz(&self) -> u32 { self.vfreq_hz }
    #[must_use]
    pub const fn hfreq_hz(&self) -> u32 { self.hfreq_hz }
    #[must_use]
    pub const fn width(&self) -> u16 { self.width }
    #[must_use]
    pub const fn height(&self) -> u16 { self.height }
    #[must_use]
    pub const fn total_h(&self) -> u16 { self.total_h }
    #[must_use]
    pub const fn total_v(&self) -> u16 { self.total_v }
    #[must_use]
    pub const fn field_rate_hz(&self) -> u32 { self.field_rate_hz }

    #[must_use]
    pub const fn from_vic(vic: u8) -> Option<Self> {
        let index = if vic >= 1 && vic <= 127 {
            (vic - 1) as usize
        } else if vic >= 193 && vic <= 219 {
            (vic - 193 + 127) as usize
        } else {
            return None;
        };
        if index < VICS.len() {
            Some(VICS[index])
        } else {
            None
        }
    }
}

#[rustfmt::skip]
static VICS: [Vic; 154] = [
    Vic { name: "DMT0659", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 25, vfreq_hz: 60, hfreq_hz: 31_469, width: 640, height: 480, total_h: 800, total_v: 525, field_rate_hz: 60 },
    Vic { name: "480p", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 8, height: 9 }, pixel_clock_mhz: 27, vfreq_hz: 60, hfreq_hz: 31_469, width: 720, height: 480, total_h: 858, total_v: 525, field_rate_hz: 60 },
    Vic { name: "480pH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 32, height: 27 }, pixel_clock_mhz: 27, vfreq_hz: 60, hfreq_hz: 31_469, width: 720, height: 480, total_h: 858, total_v: 525, field_rate_hz: 60 },
    Vic { name: "720p", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 74, vfreq_hz: 60, hfreq_hz: 45_000, width: 1280, height: 720, total_h: 1650, total_v: 750, field_rate_hz: 60 },
    Vic { name: "1080i", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 74, vfreq_hz: 60, hfreq_hz: 33_750, width: 1920, height: 540, total_h: 2200, total_v: 562, field_rate_hz: 60 },
    Vic { name: "480i", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 8, height: 9 }, pixel_clock_mhz: 27, vfreq_hz: 60, hfreq_hz: 15_734, width: 1440, height: 240, total_h: 1716, total_v: 262, field_rate_hz: 60 },
    Vic { name: "480iH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 32, height: 27 }, pixel_clock_mhz: 27, vfreq_hz: 60, hfreq_hz: 15_734, width: 1440, height: 240, total_h: 1716, total_v: 262, field_rate_hz: 60 },
    Vic { name: "240p", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 4, height: 9 }, pixel_clock_mhz: 27, vfreq_hz: 60, hfreq_hz: 15_734, width: 1440, height: 240, total_h: 1716, total_v: 262, field_rate_hz: 60 },
    Vic { name: "240pH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 16, height: 27 }, pixel_clock_mhz: 27, vfreq_hz: 60, hfreq_hz: 15_734, width: 1440, height: 240, total_h: 1716, total_v: 262, field_rate_hz: 60 },
    Vic { name: "480i4x", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 2, height: 9 }, pixel_clock_mhz: 54, vfreq_hz: 60, hfreq_hz: 15_734, width: 2880, height: 240, total_h: 3432, total_v: 262, field_rate_hz: 60 },
    Vic { name: "480i4xH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 8, height: 27 }, pixel_clock_mhz: 54, vfreq_hz: 60, hfreq_hz: 15_734, width: 2880, height: 240, total_h: 3432, total_v: 262, field_rate_hz: 60 },
    Vic { name: "240p4x", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 1, height: 9 }, pixel_clock_mhz: 54, vfreq_hz: 60, hfreq_hz: 15_734, width: 2880, height: 240, total_h: 3432, total_v: 262, field_rate_hz: 60 },
    Vic { name: "240p4xH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 4, height: 27 }, pixel_clock_mhz: 54, vfreq_hz: 60, hfreq_hz: 15_734, width: 2880, height: 240, total_h: 3432, total_v: 262, field_rate_hz: 60 },
    Vic { name: "480p2x", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 4, height: 9 }, pixel_clock_mhz: 54, vfreq_hz: 60, hfreq_hz: 31_469, width: 1440, height: 480, total_h: 1716, total_v: 525, field_rate_hz: 60 },
    Vic { name: "480p2xH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 16, height: 27 }, pixel_clock_mhz: 54, vfreq_hz: 60, hfreq_hz: 31_469, width: 1440, height: 480, total_h: 1716, total_v: 525, field_rate_hz: 60 },
    Vic { name: "1080p", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 148, vfreq_hz: 60, hfreq_hz: 67_500, width: 1920, height: 1080, total_h: 2200, total_v: 1125, field_rate_hz: 60 },
    Vic { name: "576p", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 16, height: 15 }, pixel_clock_mhz: 27, vfreq_hz: 50, hfreq_hz: 31_250, width: 720, height: 576, total_h: 864, total_v: 625, field_rate_hz: 50 },
    Vic { name: "576pH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 64, height: 45 }, pixel_clock_mhz: 27, vfreq_hz: 50, hfreq_hz: 31_250, width: 720, height: 576, total_h: 864, total_v: 625, field_rate_hz: 50 },
    Vic { name: "720p50", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 74, vfreq_hz: 50, hfreq_hz: 37_500, width: 1280, height: 720, total_h: 1980, total_v: 750, field_rate_hz: 50 },
    Vic { name: "1080i25", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 74, vfreq_hz: 50, hfreq_hz: 28_125, width: 1920, height: 540, total_h: 2640, total_v: 562, field_rate_hz: 50 },
    Vic { name: "576i", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 16, height: 15 }, pixel_clock_mhz: 27, vfreq_hz: 50, hfreq_hz: 15_625, width: 1440, height: 288, total_h: 1728, total_v: 312, field_rate_hz: 50 },
    Vic { name: "576iH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 64, height: 45 }, pixel_clock_mhz: 27, vfreq_hz: 50, hfreq_hz: 15_625, width: 1440, height: 288, total_h: 1728, total_v: 312, field_rate_hz: 50 },
    Vic { name: "288p", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 8, height: 15 }, pixel_clock_mhz: 27, vfreq_hz: 50, hfreq_hz: 15_625, width: 1440, height: 288, total_h: 1728, total_v: 313, field_rate_hz: 50 },
    Vic { name: "288pH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 32, height: 45 }, pixel_clock_mhz: 27, vfreq_hz: 50, hfreq_hz: 15_625, width: 1440, height: 288, total_h: 1728, total_v: 313, field_rate_hz: 50 },
    Vic { name: "576i4x", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 2, height: 15 }, pixel_clock_mhz: 54, vfreq_hz: 50, hfreq_hz: 15_625, width: 2880, height: 288, total_h: 3456, total_v: 312, field_rate_hz: 50 },
    Vic { name: "576i4xH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 16, height: 45 }, pixel_clock_mhz: 54, vfreq_hz: 50, hfreq_hz: 15_625, width: 2880, height: 288, total_h: 3456, total_v: 312, field_rate_hz: 50 },
    Vic { name: "288p4x", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 1, height: 15 }, pixel_clock_mhz: 54, vfreq_hz: 50, hfreq_hz: 15_625, width: 2880, height: 288, total_h: 3456, total_v: 313, field_rate_hz: 50 },
    Vic { name: "288p4xH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 8, height: 45 }, pixel_clock_mhz: 54, vfreq_hz: 50, hfreq_hz: 15_625, width: 2880, height: 288, total_h: 3456, total_v: 313, field_rate_hz: 50 },
    Vic { name: "576p2x", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 8, height: 15 }, pixel_clock_mhz: 54, vfreq_hz: 50, hfreq_hz: 31_250, width: 1440, height: 576, total_h: 1728, total_v: 625, field_rate_hz: 50 },
    Vic { name: "576p2xH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 32, height: 45 }, pixel_clock_mhz: 54, vfreq_hz: 50, hfreq_hz: 31_250, width: 1440, height: 576, total_h: 1728, total_v: 625, field_rate_hz: 50 },
    Vic { name: "1080p50", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 148, vfreq_hz: 50, hfreq_hz: 56_250, width: 1920, height: 1080, total_h: 2640, total_v: 1125, field_rate_hz: 50 },
    Vic { name: "1080p24", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 74, vfreq_hz: 24, hfreq_hz: 27_000, width: 1920, height: 1080, total_h: 2750, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "1080p25", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 74, vfreq_hz: 25, hfreq_hz: 28_125, width: 1920, height: 1080, total_h: 2640, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "1080p30", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 74, vfreq_hz: 30, hfreq_hz: 33_750, width: 1920, height: 1080, total_h: 2200, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "480p4x", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 2, height: 9 }, pixel_clock_mhz: 108, vfreq_hz: 60, hfreq_hz: 31_469, width: 2880, height: 480, total_h: 3432, total_v: 262, field_rate_hz: 60 },
    Vic { name: "480p4xH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 8, height: 27 }, pixel_clock_mhz: 108, vfreq_hz: 60, hfreq_hz: 31_469, width: 2880, height: 480, total_h: 3432, total_v: 262, field_rate_hz: 60 },
    Vic { name: "576p4x", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 4, height: 15 }, pixel_clock_mhz: 108, vfreq_hz: 50, hfreq_hz: 31_250, width: 2880, height: 576, total_h: 3456, total_v: 625, field_rate_hz: 50 },
    Vic { name: "576p4xH", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 16, height: 45 }, pixel_clock_mhz: 108, vfreq_hz: 50, hfreq_hz: 31_250, width: 2880, height: 576, total_h: 3456, total_v: 625, field_rate_hz: 50 },
    Vic { name: "1080i25", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 72, vfreq_hz: 50, hfreq_hz: 31_250, width: 1920, height: 540, total_h: 2304, total_v: 625, field_rate_hz: 50 },
    Vic { name: "1080i50", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 148, vfreq_hz: 100, hfreq_hz: 56_250, width: 1920, height: 540, total_h: 2640, total_v: 562, field_rate_hz: 100 },
    Vic { name: "720p100", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 148, vfreq_hz: 100, hfreq_hz: 45_000, width: 1280, height: 720, total_h: 1980, total_v: 750, field_rate_hz: 100 },
    Vic { name: "576p100", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 16, height: 15 }, pixel_clock_mhz: 54, vfreq_hz: 100, hfreq_hz: 62_500, width: 720, height: 576, total_h: 864, total_v: 625, field_rate_hz: 100 },
    Vic { name: "576p100H", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 64, height: 45 }, pixel_clock_mhz: 54, vfreq_hz: 100, hfreq_hz: 62_500, width: 720, height: 576, total_h: 864, total_v: 625, field_rate_hz: 100 },
    Vic { name: "576i50", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 16, height: 15 }, pixel_clock_mhz: 54, vfreq_hz: 100, hfreq_hz: 31_250, width: 1440, height: 576, total_h: 1728, total_v: 625, field_rate_hz: 100 },
    Vic { name: "576i50H", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 64, height: 45 }, pixel_clock_mhz: 54, vfreq_hz: 100, hfreq_hz: 31_250, width: 1440, height: 576, total_h: 1728, total_v: 625, field_rate_hz: 100 },
    Vic { name: "1080i60", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 148, vfreq_hz: 120, hfreq_hz: 67_500, width: 1920, height: 540, total_h: 2200, total_v: 562, field_rate_hz: 120 },
    Vic { name: "720p120", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 148, vfreq_hz: 120, hfreq_hz: 90_000, width: 1280, height: 720, total_h: 1650, total_v: 750, field_rate_hz: 120 },
    Vic { name: "480p119", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 8, height: 9 }, pixel_clock_mhz: 54, vfreq_hz: 120, hfreq_hz: 62_937, width: 720, height: 480, total_h: 858, total_v: 525, field_rate_hz: 120 },
    Vic { name: "480p119H", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 32, height: 27 }, pixel_clock_mhz: 54, vfreq_hz: 120, hfreq_hz: 62_937, width: 720, height: 480, total_h: 858, total_v: 525, field_rate_hz: 120 },
    Vic { name: "480i59", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 16, height: 15 }, pixel_clock_mhz: 54, vfreq_hz: 120, hfreq_hz: 31_469, width: 1440, height: 480, total_h: 1716, total_v: 525, field_rate_hz: 120 },
    Vic { name: "480i59H", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 64, height: 45 }, pixel_clock_mhz: 54, vfreq_hz: 120, hfreq_hz: 31_469, width: 1440, height: 480, total_h: 1716, total_v: 525, field_rate_hz: 120 },
    Vic { name: "576p200", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 16, height: 15 }, pixel_clock_mhz: 108, vfreq_hz: 200, hfreq_hz: 125_000, width: 720, height: 576, total_h: 864, total_v: 625, field_rate_hz: 200 },
    Vic { name: "576p200H", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 64, height: 45 }, pixel_clock_mhz: 108, vfreq_hz: 200, hfreq_hz: 125_000, width: 720, height: 576, total_h: 864, total_v: 625, field_rate_hz: 200 },
    Vic { name: "576i100", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 16, height: 15 }, pixel_clock_mhz: 108, vfreq_hz: 200, hfreq_hz: 62_500, width: 1440, height: 288, total_h: 1728, total_v: 312, field_rate_hz: 200 },
    Vic { name: "576i100H", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 64, height: 45 }, pixel_clock_mhz: 108, vfreq_hz: 200, hfreq_hz: 62_500, width: 1440, height: 288, total_h: 1728, total_v: 312, field_rate_hz: 200 },
    Vic { name: "480p239", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 8, height: 9 }, pixel_clock_mhz: 108, vfreq_hz: 240, hfreq_hz: 125_874, width: 720, height: 480, total_h: 858, total_v: 525, field_rate_hz: 240 },
    Vic { name: "480p239H", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 32, height: 27 }, pixel_clock_mhz: 108, vfreq_hz: 240, hfreq_hz: 125_874, width: 720, height: 480, total_h: 858, total_v: 525, field_rate_hz: 240 },
    Vic { name: "480i119", dar: AspectRatio { width: 4, height: 3 }, par: AspectRatio { width: 8, height: 9 }, pixel_clock_mhz: 108, vfreq_hz: 240, hfreq_hz: 62_937, width: 1440, height: 240, total_h: 1716, total_v: 262, field_rate_hz: 240 },
    Vic { name: "480i119H", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 32, height: 27 }, pixel_clock_mhz: 108, vfreq_hz: 240, hfreq_hz: 62_937, width: 1440, height: 240, total_h: 1716, total_v: 262, field_rate_hz: 240 },
    Vic { name: "720p24", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 59, vfreq_hz: 24, hfreq_hz: 18_000, width: 1280, height: 720, total_h: 3300, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p25", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 74, vfreq_hz: 25, hfreq_hz: 18_750, width: 1280, height: 720, total_h: 3960, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p30", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 74, vfreq_hz: 30, hfreq_hz: 22_500, width: 1280, height: 720, total_h: 3300, total_v: 750, field_rate_hz: 0 },
    Vic { name: "1080p120", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 297, vfreq_hz: 120, hfreq_hz: 135_000, width: 1920, height: 1080, total_h: 2200, total_v: 1125, field_rate_hz: 120 },
    Vic { name: "1080p100", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 297, vfreq_hz: 100, hfreq_hz: 112_500, width: 1920, height: 1080, total_h: 2640, total_v: 1125, field_rate_hz: 100 },
    Vic { name: "720p24", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 59, vfreq_hz: 24, hfreq_hz: 18_000, width: 1280, height: 720, total_h: 3300, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p25", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 74, vfreq_hz: 25, hfreq_hz: 18_750, width: 1280, height: 720, total_h: 3960, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p30", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 74, vfreq_hz: 30, hfreq_hz: 22_500, width: 1280, height: 720, total_h: 3300, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p50", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 74, vfreq_hz: 50, hfreq_hz: 37_500, width: 1280, height: 720, total_h: 1980, total_v: 750, field_rate_hz: 50 },
    Vic { name: "720p", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 74, vfreq_hz: 60, hfreq_hz: 45_000, width: 1280, height: 720, total_h: 1650, total_v: 750, field_rate_hz: 60 },
    Vic { name: "720p100", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 148, vfreq_hz: 100, hfreq_hz: 75_000, width: 1280, height: 720, total_h: 1980, total_v: 750, field_rate_hz: 100 },
    Vic { name: "720p120", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 148, vfreq_hz: 120, hfreq_hz: 90_000, width: 1280, height: 720, total_h: 1650, total_v: 750, field_rate_hz: 120 },
    Vic { name: "1080p24", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 74, vfreq_hz: 24, hfreq_hz: 27_000, width: 1920, height: 1080, total_h: 2750, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "1080p25", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 74, vfreq_hz: 25, hfreq_hz: 28_125, width: 1920, height: 1080, total_h: 2640, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "1080p30", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 74, vfreq_hz: 30, hfreq_hz: 33_750, width: 1920, height: 1080, total_h: 2200, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "1080p50", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 148, vfreq_hz: 50, hfreq_hz: 56_250, width: 1920, height: 1080, total_h: 2640, total_v: 1125, field_rate_hz: 50 },
    Vic { name: "1080p", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 148, vfreq_hz: 60, hfreq_hz: 67_500, width: 1920, height: 1080, total_h: 2200, total_v: 1125, field_rate_hz: 60 },
    Vic { name: "1080p100", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 297, vfreq_hz: 100, hfreq_hz: 112_500, width: 1920, height: 1080, total_h: 2640, total_v: 1125, field_rate_hz: 100 },
    Vic { name: "1080p120", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 297, vfreq_hz: 120, hfreq_hz: 135_000, width: 1920, height: 1080, total_h: 2200, total_v: 1125, field_rate_hz: 120 },
    Vic { name: "720p2x24", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 64, height: 63 }, pixel_clock_mhz: 59, vfreq_hz: 24, hfreq_hz: 18_000, width: 1680, height: 720, total_h: 3300, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p2x25", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 64, height: 63 }, pixel_clock_mhz: 59, vfreq_hz: 25, hfreq_hz: 18_750, width: 1680, height: 720, total_h: 3168, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p2x30", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 64, height: 63 }, pixel_clock_mhz: 59, vfreq_hz: 30, hfreq_hz: 22_500, width: 1680, height: 720, total_h: 2640, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p2x50", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 64, height: 63 }, pixel_clock_mhz: 82, vfreq_hz: 50, hfreq_hz: 37_500, width: 1680, height: 720, total_h: 2200, total_v: 750, field_rate_hz: 50 },
    Vic { name: "720p2x", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 64, height: 63 }, pixel_clock_mhz: 99, vfreq_hz: 60, hfreq_hz: 45_000, width: 1680, height: 720, total_h: 2200, total_v: 750, field_rate_hz: 60 },
    Vic { name: "720p2x100", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 64, height: 63 }, pixel_clock_mhz: 165, vfreq_hz: 100, hfreq_hz: 82_500, width: 1680, height: 720, total_h: 2000, total_v: 825, field_rate_hz: 100 },
    Vic { name: "720p2x120", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 64, height: 63 }, pixel_clock_mhz: 198, vfreq_hz: 120, hfreq_hz: 99_000, width: 1680, height: 720, total_h: 2000, total_v: 825, field_rate_hz: 120 },
    Vic { name: "1080p2x24", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 99, vfreq_hz: 24, hfreq_hz: 26_400, width: 2560, height: 1080, total_h: 3750, total_v: 1100, field_rate_hz: 0 },
    Vic { name: "1080p2x25", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 90, vfreq_hz: 25, hfreq_hz: 28_125, width: 2560, height: 1080, total_h: 3200, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "1080p2x30", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 118, vfreq_hz: 30, hfreq_hz: 33_750, width: 2560, height: 1080, total_h: 3520, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "1080p2x50", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 185, vfreq_hz: 50, hfreq_hz: 56_250, width: 2560, height: 1080, total_h: 3000, total_v: 1125, field_rate_hz: 50 },
    Vic { name: "1080p2x", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 198, vfreq_hz: 60, hfreq_hz: 66_000, width: 2560, height: 1080, total_h: 3000, total_v: 1100, field_rate_hz: 60 },
    Vic { name: "1080p2x100", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 371, vfreq_hz: 100, hfreq_hz: 125_000, width: 2560, height: 1080, total_h: 2970, total_v: 1250, field_rate_hz: 100 },
    Vic { name: "1080p2x120", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 495, vfreq_hz: 120, hfreq_hz: 150_000, width: 2560, height: 1080, total_h: 3300, total_v: 1250, field_rate_hz: 120 },
    Vic { name: "2160p24", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 297, vfreq_hz: 24, hfreq_hz: 54_000, width: 3840, height: 2160, total_h: 5500, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p25", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 297, vfreq_hz: 25, hfreq_hz: 56_250, width: 3840, height: 2160, total_h: 5280, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p30", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 297, vfreq_hz: 30, hfreq_hz: 67_500, width: 3840, height: 2160, total_h: 4400, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p50", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 594, vfreq_hz: 50, hfreq_hz: 112_500, width: 3840, height: 2160, total_h: 5280, total_v: 2250, field_rate_hz: 50 },
    Vic { name: "2160p60", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 594, vfreq_hz: 60, hfreq_hz: 135_000, width: 3840, height: 2160, total_h: 4400, total_v: 2250, field_rate_hz: 60 },
    Vic { name: "2160p24", dar: AspectRatio { width: 256, height: 135 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 297, vfreq_hz: 24, hfreq_hz: 67_500, width: 4096, height: 2160, total_h: 5500, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p25", dar: AspectRatio { width: 256, height: 135 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 297, vfreq_hz: 25, hfreq_hz: 112_500, width: 4096, height: 2160, total_h: 5280, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p30", dar: AspectRatio { width: 256, height: 135 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 297, vfreq_hz: 30, hfreq_hz: 135_000, width: 4096, height: 2160, total_h: 4400, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p50", dar: AspectRatio { width: 256, height: 135 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 594, vfreq_hz: 50, hfreq_hz: 112_500, width: 4096, height: 2160, total_h: 5280, total_v: 2250, field_rate_hz: 50 },
    Vic { name: "2160p", dar: AspectRatio { width: 256, height: 135 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 594, vfreq_hz: 60, hfreq_hz: 135_000, width: 4096, height: 2160, total_h: 4400, total_v: 2250, field_rate_hz: 60 },
    Vic { name: "2160p24", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 297, vfreq_hz: 24, hfreq_hz: 67_500, width: 3840, height: 2160, total_h: 5500, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p25", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 297, vfreq_hz: 25, hfreq_hz: 112_500, width: 3840, height: 2160, total_h: 5280, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p30", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 297, vfreq_hz: 30, hfreq_hz: 135_000, width: 3840, height: 2160, total_h: 4400, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p50", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 594, vfreq_hz: 50, hfreq_hz: 112_500, width: 3840, height: 2160, total_h: 5280, total_v: 2250, field_rate_hz: 50 },
    Vic { name: "2160p", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 594, vfreq_hz: 60, hfreq_hz: 135_000, width: 3840, height: 2160, total_h: 4400, total_v: 2250, field_rate_hz: 60 },
    Vic { name: "720p48", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 90, vfreq_hz: 48, hfreq_hz: 36_000, width: 1280, height: 720, total_h: 2500, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p48", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 90, vfreq_hz: 48, hfreq_hz: 36_000, width: 1280, height: 720, total_h: 2500, total_v: 750, field_rate_hz: 0 },
    Vic { name: "720p2x48", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 64, height: 63 }, pixel_clock_mhz: 99, vfreq_hz: 48, hfreq_hz: 36_000, width: 1680, height: 720, total_h: 2750, total_v: 825, field_rate_hz: 0 },
    Vic { name: "1080p48", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 148, vfreq_hz: 48, hfreq_hz: 54_000, width: 1920, height: 1080, total_h: 2750, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "1080p48", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 148, vfreq_hz: 48, hfreq_hz: 54_000, width: 1920, height: 1080, total_h: 2750, total_v: 1125, field_rate_hz: 0 },
    Vic { name: "1080p2x48", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 198, vfreq_hz: 48, hfreq_hz: 52_800, width: 2560, height: 1080, total_h: 3750, total_v: 1100, field_rate_hz: 0 },
    Vic { name: "2160p48", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 594, vfreq_hz: 48, hfreq_hz: 108_000, width: 3840, height: 2160, total_h: 5500, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p48", dar: AspectRatio { width: 256, height: 135 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 594, vfreq_hz: 48, hfreq_hz: 108_000, width: 4096, height: 2160, total_h: 5500, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p48", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 594, vfreq_hz: 48, hfreq_hz: 108_000, width: 3840, height: 2160, total_h: 5500, total_v: 2250, field_rate_hz: 0 },
    Vic { name: "2160p100", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1188, vfreq_hz: 100, hfreq_hz: 225_000, width: 3840, height: 2160, total_h: 5280, total_v: 2250, field_rate_hz: 100 },
    Vic { name: "2160p120", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1188, vfreq_hz: 120, hfreq_hz: 270_000, width: 3840, height: 2160, total_h: 4400, total_v: 2250, field_rate_hz: 120 },
    Vic { name: "2160p100", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 1188, vfreq_hz: 100, hfreq_hz: 225_000, width: 3840, height: 2160, total_h: 5280, total_v: 2250, field_rate_hz: 100 },
    Vic { name: "2160p120", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 1188, vfreq_hz: 120, hfreq_hz: 270_000, width: 3840, height: 2160, total_h: 4400, total_v: 2250, field_rate_hz: 120 },
    Vic { name: "2160p2x24", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 396, vfreq_hz: 24, hfreq_hz: 52_800, width: 5120, height: 2160, total_h: 7500, total_v: 2200, field_rate_hz: 0 },
    Vic { name: "2160p2x25", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 396, vfreq_hz: 25, hfreq_hz: 55_000, width: 5120, height: 2160, total_h: 7200, total_v: 2200, field_rate_hz: 0 },
    Vic { name: "2160p2x30", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 396, vfreq_hz: 30, hfreq_hz: 66_000, width: 5120, height: 2160, total_h: 6000, total_v: 2200, field_rate_hz: 0 },
    Vic { name: "2160p2x48", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 742, vfreq_hz: 48, hfreq_hz: 118_800, width: 5120, height: 2160, total_h: 6250, total_v: 2450, field_rate_hz: 0 },
    Vic { name: "2160p2x50", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 742, vfreq_hz: 50, hfreq_hz: 112_500, width: 5120, height: 2160, total_h: 6600, total_v: 2250, field_rate_hz: 50 },
    Vic { name: "2160p2x", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 742, vfreq_hz: 60, hfreq_hz: 135_000, width: 5120, height: 2160, total_h: 5500, total_v: 2250, field_rate_hz: 60 },
    Vic { name: "2160p2x100", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1485, vfreq_hz: 100, hfreq_hz: 225_000, width: 5120, height: 2160, total_h: 6600, total_v: 2250, field_rate_hz: 100 },
    Vic { name: "2160p2x120", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1485, vfreq_hz: 120, hfreq_hz: 270_000, width: 5120, height: 2160, total_h: 5500, total_v: 2250, field_rate_hz: 120 },
    Vic { name: "4320p24", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1188, vfreq_hz: 24, hfreq_hz: 108_000, width: 7680, height: 4320, total_h: 11000, total_v: 4500, field_rate_hz: 0 },
    Vic { name: "4320p25", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1188, vfreq_hz: 25, hfreq_hz: 110_000, width: 7680, height: 4320, total_h: 10800, total_v: 4400, field_rate_hz: 0 },
    Vic { name: "4320p30", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1188, vfreq_hz: 30, hfreq_hz: 132_000, width: 7680, height: 4320, total_h: 9000, total_v: 4400, field_rate_hz: 0 },
    Vic { name: "4320p48", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 2376, vfreq_hz: 48, hfreq_hz: 216_000, width: 7680, height: 4320, total_h: 11000, total_v: 4500, field_rate_hz: 0 },
    Vic { name: "4320p50", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 2376, vfreq_hz: 50, hfreq_hz: 220_000, width: 7680, height: 4320, total_h: 10800, total_v: 4400, field_rate_hz: 50 },
    Vic { name: "4320p", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 2376, vfreq_hz: 60, hfreq_hz: 264_000, width: 7680, height: 4320, total_h: 9000, total_v: 4400, field_rate_hz: 60 },
    Vic { name: "4320p100", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 4752, vfreq_hz: 100, hfreq_hz: 450_000, width: 7680, height: 4320, total_h: 10560, total_v: 4500, field_rate_hz: 100 },
    Vic { name: "4320p120", dar: AspectRatio { width: 16, height: 9 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 4752, vfreq_hz: 120, hfreq_hz: 540_000, width: 7680, height: 4320, total_h: 8800, total_v: 4500, field_rate_hz: 120 },
    Vic { name: "4320p24", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 1188, vfreq_hz: 24, hfreq_hz: 108_000, width: 7680, height: 4320, total_h: 11000, total_v: 4500, field_rate_hz: 0 },
    Vic { name: "4320p25", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 1188, vfreq_hz: 25, hfreq_hz: 110_000, width: 7680, height: 4320, total_h: 10800, total_v: 4400, field_rate_hz: 0 },
    Vic { name: "4320p30", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 1188, vfreq_hz: 30, hfreq_hz: 132_000, width: 7680, height: 4320, total_h: 9000, total_v: 4400, field_rate_hz: 0 },
    Vic { name: "4320p48", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 2376, vfreq_hz: 48, hfreq_hz: 216_000, width: 7680, height: 4320, total_h: 11000, total_v: 4500, field_rate_hz: 0 },
    Vic { name: "4320p50", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 2376, vfreq_hz: 50, hfreq_hz: 220_000, width: 7680, height: 4320, total_h: 10800, total_v: 4400, field_rate_hz: 50 },
    Vic { name: "4320p", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 2376, vfreq_hz: 60, hfreq_hz: 264_000, width: 7680, height: 4320, total_h: 9000, total_v: 4400, field_rate_hz: 60 },
    Vic { name: "4320p100", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 4752, vfreq_hz: 100, hfreq_hz: 450_000, width: 7680, height: 4320, total_h: 10560, total_v: 4500, field_rate_hz: 100 },
    Vic { name: "4320p120", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 4, height: 3 }, pixel_clock_mhz: 4752, vfreq_hz: 120, hfreq_hz: 540_000, width: 7680, height: 4320, total_h: 8800, total_v: 4500, field_rate_hz: 120 },
    Vic { name: "4320p2x24", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1485, vfreq_hz: 24, hfreq_hz: 118_800, width: 10240, height: 4320, total_h: 12500, total_v: 4950, field_rate_hz: 0 },
    Vic { name: "4320p2x25", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1485, vfreq_hz: 25, hfreq_hz: 110_000, width: 10240, height: 4320, total_h: 13500, total_v: 4400, field_rate_hz: 0 },
    Vic { name: "4320p2x30", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1485, vfreq_hz: 30, hfreq_hz: 135_000, width: 10240, height: 4320, total_h: 11000, total_v: 4500, field_rate_hz: 0 },
    Vic { name: "4320p2x48", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 2970, vfreq_hz: 48, hfreq_hz: 237_600, width: 10240, height: 4320, total_h: 12500, total_v: 4950, field_rate_hz: 0 },
    Vic { name: "4320p2x50", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 2970, vfreq_hz: 50, hfreq_hz: 220_000, width: 10240, height: 4320, total_h: 13500, total_v: 4400, field_rate_hz: 50 },
    Vic { name: "4320p2x", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 2970, vfreq_hz: 60, hfreq_hz: 270_000, width: 10240, height: 4320, total_h: 11000, total_v: 4400, field_rate_hz: 60 },
    Vic { name: "4320p2x100", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 5940, vfreq_hz: 100, hfreq_hz: 450_000, width: 10240, height: 4320, total_h: 13200, total_v: 4500, field_rate_hz: 100 },
    Vic { name: "4320p2x120", dar: AspectRatio { width: 64, height: 27 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 5940, vfreq_hz: 120, hfreq_hz: 540_000, width: 10240, height: 4320, total_h: 11000, total_v: 4500, field_rate_hz: 120 },
    Vic { name: "2160p100", dar: AspectRatio { width: 256, height: 135 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1188, vfreq_hz: 100, hfreq_hz: 225_000, width: 4096, height: 2160, total_h: 5280, total_v: 2250, field_rate_hz: 100 },
    Vic { name: "2160p120", dar: AspectRatio { width: 256, height: 135 }, par: AspectRatio { width: 1, height: 1 }, pixel_clock_mhz: 1188, vfreq_hz: 120, hfreq_hz: 270_000, width: 4096, height: 2160, total_h: 4400, total_v: 2250, field_rate_hz: 120 },
];
