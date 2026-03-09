//! CTA Vendor Specific Data Blocks.
//!
//! Contains vendor-specific information, most commonly the HDMI
//! Vendor Specific Data Block (VSDB) with IEEE registration number
//! 0x00000C03 (HDMI Licensing Administrator).
//!
//! # HDMI VSDB Structure
//!
//! | Byte(s) | Description |
//! |---------|-------------|
//! | 0–2     | IEEE OUI (0x00000C03 little-endian) |
//! | 3–4     | Physical address (4 nibbles) |
//! | 5       | Deep color support flags |
//! | 6       | Max TMDS clock (×5 MHz) |
//! | 7+      | Latency and other flags |

/// HDMI Vendor Specific Data Block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HdmiVsdb {
    org_uid: u32,
    phys_addr: (u8, u8, u8, u8),
    deep_color: u8,
    max_tmds_mhz: Option<u16>,
    latency: u8,
    video_lat: Option<u16>,
    audio_lat: Option<u16>,
    video_int_lat: Option<u16>,
    audio_int_lat: Option<u16>,
}

#[rustfmt::skip]
impl HdmiVsdb {
    #[must_use]
    pub const fn oui(&self) -> u32 { self.org_uid }
    #[must_use]
    pub const fn phys_addr(&self) -> (u8, u8, u8, u8) { self.phys_addr }
    #[must_use]
    pub const fn ai(&self) -> bool { (self.deep_color & 0x80) != 0 }
    #[must_use]
    pub const fn dc_48(&self) -> bool { (self.deep_color & 0x40) != 0 }
    #[must_use]
    pub const fn dc_36(&self) -> bool { (self.deep_color & 0x20) != 0 }
    #[must_use]
    pub const fn dc_30(&self) -> bool { (self.deep_color & 0x10) != 0 }
    #[must_use]
    pub const fn dc_444(&self) -> bool { (self.deep_color & 0x08) != 0 }
    #[must_use]
    pub const fn dvi_dual(&self) -> bool { (self.deep_color & 0x01) != 0 }
    #[must_use]
    pub const fn max_tmds_mhz(&self) -> Option<u16> { self.max_tmds_mhz }
    #[must_use]
    pub const fn lat_present(&self) -> bool { (self.latency & 0x80) != 0 }
    #[must_use]
    pub const fn ilat_present(&self) -> bool { (self.latency & 0x40) != 0 }
    #[must_use]
    pub const fn video_lat_ms(&self) -> Option<u16> { self.video_lat }
    #[must_use]
    pub const fn audio_lat_ms(&self) -> Option<u16> { self.audio_lat }
    #[must_use]
    pub const fn interlaced_video_lat_ms(&self) -> Option<u16> { self.video_int_lat }
    #[must_use]
    pub const fn interlaced_audio_lat_ms(&self) -> Option<u16> { self.audio_int_lat }

    #[must_use]
    pub fn parse(raw: &[u8]) -> Option<Self> {
        if raw.len() < 5 {
            return None;
        }
        let org_uid = u32::from(raw[0]) | (u32::from(raw[1]) << 8) | (u32::from(raw[2]) << 16);
        if org_uid != 0x0000_0c03 {
            return None;
        }
        let phys_addr = (raw[3] >> 4, raw[3] & 15, raw[4] >> 4, raw[4] & 15);
        let deep_color = raw.get(5).copied().unwrap_or(0);
        let max_tmds_mhz = raw
            .get(6)
            .copied()
            .filter(|v| *v != 0)
            .map(|v| u16::from(v) * 5);

        let latency = raw.get(7).copied().unwrap_or(0) & 0b1100_0000;
        let has_lat = (latency & 0b1000_0000) != 0;
        let has_int_lat = has_lat && (latency & 0b0100_0000) != 0;

        let video_lat = has_lat
            .then(|| raw.get(8).copied().and_then(lat_ms))
            .flatten();
        let audio_lat = has_lat
            .then(|| raw.get(9).copied().and_then(lat_ms))
            .flatten();
        let video_int_lat = has_int_lat
            .then(|| raw.get(10).copied().and_then(lat_ms))
            .flatten();
        let audio_int_lat = has_int_lat
            .then(|| raw.get(11).copied().and_then(lat_ms))
            .flatten();
        Some(Self {
            org_uid,
            phys_addr,
            deep_color,
            max_tmds_mhz,
            latency,
            video_lat,
            audio_lat,
            video_int_lat,
            audio_int_lat,
        })
    }
}

/// Converts HDMI VSDB latency field value to milliseconds.
///
/// Per HDMI CTA-861 spec, the latency field encoding is:
/// - `0` → latency not specified (returns `None`)
/// - `1..=250` → `(value - 1) * 2` ms (i.e., 0 ms to 498 ms in 2 ms steps)
/// - `251` → 500 ms (maximum representable latency)
/// - `252..=255` → reserved/invalid (returns `None`)
///
/// # References
/// - CTA-861-G, Section 6.4.5: Vendor Specific Data Block
const fn lat_ms(raw: u8) -> Option<u16> {
    if raw == 0 {
        None
    } else if raw == 251 {
        Some(500)
    } else if raw < 251 {
        Some(((raw as u16) - 1) * 2)
    } else {
        None
    }
}
