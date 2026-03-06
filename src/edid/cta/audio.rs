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

impl AudioFormat {
    #[must_use]
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

impl AudioExtFormat {
    #[must_use]
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

/// Short Audio Descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sad {
    fmt: AudioFormat,
    ch: u8,
    rates: u8,
    byte3: u8,
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
            | AudioFormat::Atrac => Some(self.byte3 as u16 * 8),
            _ => None,
        }
    }

    #[must_use]
    pub const fn parse(raw0: u8, raw1: u8, raw2: u8) -> Self {
        Self {
            fmt: AudioFormat::parse((raw0 >> 3) & 0b1111),
            ch: (raw0 & 0b0111) + 1,
            rates: raw1 & 0b0111_1111,
            byte3: raw2,
        }
    }
}

/// Iterator over SADs in an Audio Data Block.
pub struct SadIter<'a> {
    pub raw: &'a [u8],
    pub at: usize,
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
