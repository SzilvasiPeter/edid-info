use crate::edid::descriptor::timing::{DETAILED_LEN, DetailedTiming};

pub const CTA_LEN: usize = 128;
pub const CTA_TAG: u8 = 0b0000_0010;

// CTA data block length is a 5-bit field (bits 4..0), so max payload is (2^5 - 1) 31 bytes.
const DB_MAX_LEN: usize = 31;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockTag {
    Audio,
    Video,
    Vendor,
    Speaker,
    VesaDtc,
    VideoFmt,
    Extended,
    Reserved(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataBlock {
    tag: BlockTag,
    ext_tag: Option<u8>,
    len: u8,
    data: [u8; DB_MAX_LEN],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Svd {
    vic: u8,
    native: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioFormat {
    Reserved,
    Lpcm,
    Ac3,
    Mpeg1,
    Mp3,
    Mpeg2,
    AacLc,
    Dts,
    Atrac,
    Dsd,
    DdPlus,
    DtsHd,
    MatMlp,
    Dst,
    WmaPro,
    Ext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioExtFormat {
    Reserved,
    Mpeg4HeAac,
    Mpeg4HeAacV2,
    Mpeg4AacLc,
    Dra,
    Mpeg4HeAacMps,
    Mpeg4HeAacLcMps,
    MpegH3d,
    Ac4,
    Lpcm3d,
    AuroCx,
    MpegDUsac,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sad {
    fmt: AudioFormat,
    ch: u8,
    rates: u8,
    byte3: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HdmiVsdb {
    org_uid: u32,
    phys_addr: (u8, u8, u8, u8),
    deep_color: u8,
    max_tmds_mhz: Option<u16>,
    lat: u8,
    video_lat: Option<u16>,
    audio_lat: Option<u16>,
    video_ilat: Option<u16>,
    audio_ilat: Option<u16>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vic {
    name: &'static str,
    width: u16,
    height: u16,
    vfreq_millihz: u32,
}

impl Vic {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }
    #[must_use]
    pub const fn vfreq_millihz(&self) -> u32 {
        self.vfreq_millihz
    }
    #[must_use]
    pub const fn from_vic(vic: u8) -> Option<Self> {
        match vic {
            1 => Some(Self {
                name: "DMT0659",
                width: 640,
                height: 480,
                vfreq_millihz: 59_940,
            }),
            3 => Some(Self {
                name: "480pH",
                width: 720,
                height: 480,
                vfreq_millihz: 59_940,
            }),
            4 => Some(Self {
                name: "720p",
                width: 1280,
                height: 720,
                vfreq_millihz: 60_000,
            }),
            16 => Some(Self {
                name: "1080p",
                width: 1920,
                height: 1080,
                vfreq_millihz: 60_000,
            }),
            18 => Some(Self {
                name: "576pH",
                width: 720,
                height: 576,
                vfreq_millihz: 50_000,
            }),
            19 => Some(Self {
                name: "720p50",
                width: 1280,
                height: 720,
                vfreq_millihz: 50_000,
            }),
            31 => Some(Self {
                name: "1080p50",
                width: 1920,
                height: 1080,
                vfreq_millihz: 50_000,
            }),
            93 => Some(Self {
                name: "2160p24",
                width: 3840,
                height: 2160,
                vfreq_millihz: 24_000,
            }),
            94 => Some(Self {
                name: "2160p25",
                width: 3840,
                height: 2160,
                vfreq_millihz: 25_000,
            }),
            95 => Some(Self {
                name: "2160p30",
                width: 3840,
                height: 2160,
                vfreq_millihz: 30_000,
            }),
            96 => Some(Self {
                name: "2160p50",
                width: 3840,
                height: 2160,
                vfreq_millihz: 50_000,
            }),
            97 => Some(Self {
                name: "2160p60",
                width: 3840,
                height: 2160,
                vfreq_millihz: 60_000,
            }),
            _ => None,
        }
    }
}

impl Svd {
    #[must_use]
    pub const fn vic(&self) -> u8 {
        self.vic
    }

    #[must_use]
    pub const fn native(&self) -> bool {
        self.native
    }
    #[must_use]
    pub const fn timing(&self) -> Option<Vic> {
        Vic::from_vic(self.vic)
    }

    const fn parse(raw: u8) -> Self {
        let lo = raw & 0b0111_1111;
        let hi = (raw & 0b1000_0000) != 0;
        let native = hi && lo > 0 && lo <= 64;
        let vic = if hi && lo >= 65 { lo | 0b1000_0000 } else { lo };
        Self { vic, native }
    }
}

pub struct SvdIter<'a> {
    raw: &'a [u8],
    at: usize,
}

pub struct SadIter<'a> {
    raw: &'a [u8],
    at: usize,
}

impl AudioFormat {
    const fn parse(raw: u8) -> Self {
        const MAP: [AudioFormat; 16] = [
            AudioFormat::Reserved,
            AudioFormat::Lpcm,
            AudioFormat::Ac3,
            AudioFormat::Mpeg1,
            AudioFormat::Mp3,
            AudioFormat::Mpeg2,
            AudioFormat::AacLc,
            AudioFormat::Dts,
            AudioFormat::Atrac,
            AudioFormat::Dsd,
            AudioFormat::DdPlus,
            AudioFormat::DtsHd,
            AudioFormat::MatMlp,
            AudioFormat::Dst,
            AudioFormat::WmaPro,
            AudioFormat::Ext,
        ];
        MAP[(raw & 0b1111) as usize]
    }
}

impl AudioExtFormat {
    const fn parse(raw: u8) -> Self {
        match raw {
            4 => Self::Mpeg4HeAac,
            5 => Self::Mpeg4HeAacV2,
            6 => Self::Mpeg4AacLc,
            7 => Self::Dra,
            8 => Self::Mpeg4HeAacMps,
            10 => Self::Mpeg4HeAacLcMps,
            11 => Self::MpegH3d,
            12 => Self::Ac4,
            13 => Self::Lpcm3d,
            14 => Self::AuroCx,
            15 => Self::MpegDUsac,
            _ => Self::Reserved,
        }
    }
}

impl Sad {
    #[must_use]
    pub const fn format(&self) -> AudioFormat {
        self.fmt
    }
    #[must_use]
    pub const fn channels(&self) -> u8 {
        self.ch
    }
    #[must_use]
    pub const fn rates(&self) -> u8 {
        self.rates
    }
    #[must_use]
    pub const fn has_rate(&self, khz: u16) -> bool {
        let bit = match khz {
            32 => 0,
            44 => 1,
            48 => 2,
            88 => 3,
            96 => 4,
            176 => 5,
            192 => 6,
            _ => 7,
        };
        bit < 7 && ((self.rates >> bit) & 1) != 0
    }
    #[must_use]
    pub const fn ext(&self) -> Option<AudioExtFormat> {
        if matches!(self.fmt, AudioFormat::Ext) {
            Some(AudioExtFormat::parse(self.byte3 >> 3))
        } else {
            None
        }
    }
    #[must_use]
    pub const fn lpcm_depth(&self) -> u8 {
        if matches!(self.fmt, AudioFormat::Lpcm) {
            self.byte3 & 0b0000_0111
        } else {
            0
        }
    }
    #[must_use]
    pub const fn max_kbps(&self) -> Option<u16> {
        match self.fmt {
            AudioFormat::Ac3
            | AudioFormat::Mpeg1
            | AudioFormat::Mp3
            | AudioFormat::Mpeg2
            | AudioFormat::AacLc
            | AudioFormat::Dts
            | AudioFormat::Atrac => {
                // SAD byte3 stores bitrate/8000 (bit/s), so kbps is byte3*8000/1000 = byte3*8.
                Some(self.byte3 as u16 * 8)
            }
            _ => None,
        }
    }
    const fn parse(raw0: u8, raw1: u8, raw2: u8) -> Self {
        Self {
            fmt: AudioFormat::parse((raw0 >> 3) & 0b1111),
            ch: (raw0 & 0b0111) + 1,
            rates: raw1 & 0b0111_1111,
            byte3: raw2,
        }
    }
}

impl HdmiVsdb {
    #[must_use]
    pub const fn oui(&self) -> u32 {
        self.org_uid
    }
    #[must_use]
    pub const fn phys_addr(&self) -> (u8, u8, u8, u8) {
        self.phys_addr
    }
    #[must_use]
    pub const fn ai(&self) -> bool {
        (self.deep_color & 0b1000_0000) != 0
    }
    #[must_use]
    pub const fn dc_48(&self) -> bool {
        (self.deep_color & 0b0100_0000) != 0
    }
    #[must_use]
    pub const fn dc_36(&self) -> bool {
        (self.deep_color & 0b0010_0000) != 0
    }
    #[must_use]
    pub const fn dc_30(&self) -> bool {
        (self.deep_color & 0b0001_0000) != 0
    }
    #[must_use]
    pub const fn dc_444(&self) -> bool {
        (self.deep_color & 0b0000_1000) != 0
    }
    #[must_use]
    pub const fn dvi_dual(&self) -> bool {
        (self.deep_color & 0b0000_0001) != 0
    }
    #[must_use]
    pub const fn max_tmds_mhz(&self) -> Option<u16> {
        self.max_tmds_mhz
    }
    #[must_use]
    pub const fn lat_present(&self) -> bool {
        (self.lat & 0b1000_0000) != 0
    }
    #[must_use]
    pub const fn ilat_present(&self) -> bool {
        (self.lat & 0b0100_0000) != 0
    }
    #[must_use]
    pub const fn video_lat_ms(&self) -> Option<u16> {
        self.video_lat
    }
    #[must_use]
    pub const fn audio_lat_ms(&self) -> Option<u16> {
        self.audio_lat
    }
    #[must_use]
    pub const fn interlaced_video_lat_ms(&self) -> Option<u16> {
        self.video_ilat
    }
    #[must_use]
    pub const fn interlaced_audio_lat_ms(&self) -> Option<u16> {
        self.audio_ilat
    }
}

impl DataBlock {
    #[must_use]
    pub const fn tag(&self) -> BlockTag {
        self.tag
    }
    #[must_use]
    pub const fn ext_tag(&self) -> Option<u8> {
        self.ext_tag
    }
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data[..usize::from(self.len)]
    }

    #[must_use]
    pub fn svd(&self, i: usize) -> Option<Svd> {
        if self.tag != BlockTag::Video {
            return None;
        }
        self.data().get(i).copied().map(Svd::parse)
    }

    #[must_use]
    pub fn svds(&self) -> SvdIter<'_> {
        let raw = if self.tag == BlockTag::Video {
            self.data()
        } else {
            &[]
        };
        SvdIter { raw, at: 0 }
    }

    #[must_use]
    pub fn sad(&self, i: usize) -> Option<Sad> {
        if self.tag != BlockTag::Audio {
            return None;
        }
        let at = i * 3;
        let raw = self.data();
        if at + 2 >= raw.len() {
            return None;
        }
        Some(Sad::parse(raw[at], raw[at + 1], raw[at + 2]))
    }

    #[must_use]
    pub fn sads(&self) -> SadIter<'_> {
        let raw = if self.tag == BlockTag::Audio {
            self.data()
        } else {
            &[]
        };
        SadIter { raw, at: 0 }
    }

    #[must_use]
    pub fn vendor_oui(&self) -> Option<u32> {
        if self.tag != BlockTag::Vendor {
            return None;
        }
        let raw = self.data();
        if raw.len() < 3 {
            return None;
        }
        Some(u32::from(raw[0]) | (u32::from(raw[1]) << 8) | (u32::from(raw[2]) << 16))
    }

    #[must_use]
    pub fn hdmi_vsdb(&self) -> Option<HdmiVsdb> {
        let raw = self.data();
        if self.tag != BlockTag::Vendor || raw.len() < 5 {
            return None;
        }
        let oui = self.vendor_oui()?;
        if oui != 0x0000_0c03 {
            return None;
        }
        let pa0 = raw[3] >> 4;
        let pa1 = raw[3] & 0x0f;
        let pa2 = raw[4] >> 4;
        let pa3 = raw[4] & 0x0f;
        let dc = raw.get(5).copied().unwrap_or(0);
        let max_tmds_mhz = raw
            .get(6)
            .copied()
            .filter(|v| *v != 0)
            .map(|v| u16::from(v) * 5);
        let lat = raw.get(7).copied().unwrap_or(0) & 0b1100_0000;
        let has_lat = (lat & 0b1000_0000) != 0;
        let has_int_lat = has_lat && (lat & 0b0100_0000) != 0;
        let v_ms = has_lat
            .then(|| raw.get(8).copied().and_then(lat_ms))
            .flatten();
        let a_ms = has_lat
            .then(|| raw.get(9).copied().and_then(lat_ms))
            .flatten();
        let video_latency = has_int_lat
            .then(|| raw.get(10).copied().and_then(lat_ms))
            .flatten();
        let audio_latency = has_int_lat
            .then(|| raw.get(11).copied().and_then(lat_ms))
            .flatten();
        Some(HdmiVsdb {
            org_uid: oui,
            phys_addr: (pa0, pa1, pa2, pa3),
            deep_color: dc,
            max_tmds_mhz,
            lat,
            video_lat: v_ms,
            audio_lat: a_ms,
            video_ilat: video_latency,
            audio_ilat: audio_latency,
        })
    }
}

impl Iterator for SvdIter<'_> {
    type Item = Svd;
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.raw.get(self.at).copied().map(Svd::parse);
        self.at += usize::from(out.is_some());
        out
    }
}

impl Iterator for SadIter<'_> {
    type Item = Sad;
    fn next(&mut self) -> Option<Self::Item> {
        let at = self.at;
        let raw = self.raw;
        if at + 2 >= raw.len() {
            return None;
        }
        self.at = at + 3;
        Some(Sad::parse(raw[at], raw[at + 1], raw[at + 2]))
    }
}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cta {
    raw: [u8; CTA_LEN],
}

impl Cta {
    #[must_use]
    pub fn parse(raw: &[u8; CTA_LEN]) -> Option<Self> {
        (raw[0] == CTA_TAG).then_some(Self { raw: *raw })
    }

    #[must_use]
    pub const fn rev(&self) -> u8 {
        self.raw[1]
    }
    #[must_use]
    pub const fn native_dtd_num(&self) -> u8 {
        self.raw[3] & 0b0000_1111
    }
    #[must_use]
    pub const fn underscan(&self) -> bool {
        (self.raw[3] & 0b1000_0000) != 0
    }
    #[must_use]
    pub const fn basic_audio(&self) -> bool {
        (self.raw[3] & 0b0100_0000) != 0
    }
    #[must_use]
    pub const fn ycbcr_444(&self) -> bool {
        (self.raw[3] & 0b0010_0000) != 0
    }
    #[must_use]
    pub const fn ycbcr_422(&self) -> bool {
        (self.raw[3] & 0b0001_0000) != 0
    }
    #[must_use]
    pub const fn checksum(&self) -> u8 {
        self.raw[127]
    }
    #[must_use]
    pub fn checksum_ok(&self) -> bool {
        self.raw.iter().fold(0u8, |sum, b| sum.wrapping_add(*b)) == 0
    }

    #[must_use]
    pub const fn data_blocks(&self) -> DataBlockIter<'_> {
        DataBlockIter {
            raw: &self.raw,
            at: 4,
            end: self.dbc_end(),
        }
    }

    #[must_use]
    pub fn dtd(&self, i: usize) -> Option<DetailedTiming> {
        let start = self.dtd_start()?;
        let off = start + i * DETAILED_LEN;
        let end = off + DETAILED_LEN;
        if end > 127 || (self.raw[off] == 0 && self.raw[off + 1] == 0) {
            return None;
        }
        let mut raw = [0; DETAILED_LEN];
        raw.copy_from_slice(&self.raw[off..end]);
        DetailedTiming::parse(&raw)
    }

    const fn dbc_end(&self) -> usize {
        let dtd_off = self.raw[2];
        if dtd_off == 0 {
            127
        } else if dtd_off >= 4 {
            dtd_off as usize
        } else {
            4
        }
    }

    const fn dtd_start(&self) -> Option<usize> {
        let dtd_off = self.raw[2];
        if dtd_off >= 4 && dtd_off < 127 {
            Some(dtd_off as usize)
        } else {
            None
        }
    }
}

pub struct DataBlockIter<'a> {
    raw: &'a [u8; CTA_LEN],
    at: usize,
    end: usize,
}

impl Iterator for DataBlockIter<'_> {
    type Item = DataBlock;
    fn next(&mut self) -> Option<Self::Item> {
        if self.at >= self.end {
            return None;
        }
        let block_header = self.raw[self.at];
        let tag_raw = block_header >> 5;
        let len = usize::from(block_header & 0b0001_1111);
        let next = self.at + 1 + len;
        if next > self.end {
            self.at = self.end;
            return None;
        }
        let mut data = [0; DB_MAX_LEN];
        data[..len].copy_from_slice(&self.raw[self.at + 1..next]);
        self.at = next;
        let tag = match tag_raw {
            1 => BlockTag::Audio,
            2 => BlockTag::Video,
            3 => BlockTag::Vendor,
            4 => BlockTag::Speaker,
            5 => BlockTag::VesaDtc,
            6 => BlockTag::VideoFmt,
            7 => BlockTag::Extended,
            _ => BlockTag::Reserved(tag_raw),
        };
        let ext_tag = (tag_raw == 7 && len > 0).then_some(data[0]);
        Some(DataBlock {
            tag,
            ext_tag,
            len: block_header & 0b0001_1111,
            data,
        })
    }
}
