use crate::edid::DESC_LEN;
use crate::edid::descriptor::color::Color;
use crate::edid::descriptor::cvt3::Cvt3;
use crate::edid::descriptor::range::Range;
use crate::edid::descriptor::std2::Std2;
use crate::edid::descriptor::std3::Std3;
use crate::edid::descriptor::white_point::WhitePoint;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DescTag {
    SerialNumber,
    Text,
    RangeLimits,
    MonitorName,
    WhitePoint,
    StdTimings2,
    Dcm,
    Cvt3Byte,
    StdTiming3,
    Dummy,
    VendorReserved,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MonitorDesc {
    tag: DescTag,
    tag_raw: u8,
    byte4: u8,
    data: [u8; 13],
}

impl MonitorDesc {
    #[must_use]
    pub fn parse(raw: &[u8; DESC_LEN]) -> Option<Self> {
        if raw[0] != 0 || raw[1] != 0 || raw[2] != 0 {
            return None;
        }
        let tag = match raw[3] {
            0xFF => DescTag::SerialNumber,
            0xFE => DescTag::Text,
            0xFD => DescTag::RangeLimits,
            0xFC => DescTag::MonitorName,
            0xFB => DescTag::WhitePoint,
            0xFA => DescTag::StdTimings2,
            0xF9 => DescTag::Dcm,
            0xF8 => DescTag::Cvt3Byte,
            0xF7 => DescTag::StdTiming3,
            0x10 => DescTag::Dummy,
            0x00..=0x0F => DescTag::VendorReserved,
            _ => DescTag::Unknown,
        };
        if !matches!(tag, DescTag::RangeLimits) && raw[4] != 0 {
            return None;
        }
        let mut data = [0; 13];
        data.copy_from_slice(&raw[5..DESC_LEN]);
        Some(Self {
            tag,
            tag_raw: raw[3],
            byte4: raw[4],
            data,
        })
    }

    #[must_use]
    pub const fn tag(&self) -> DescTag {
        self.tag
    }

    #[must_use]
    pub const fn data(&self) -> &[u8] {
        &self.data
    }

    fn raw_desc(&self) -> [u8; DESC_LEN] {
        let mut raw = [0; DESC_LEN];
        raw[3] = self.tag_raw;
        raw[4] = self.byte4;
        raw[5..DESC_LEN].copy_from_slice(&self.data);
        raw
    }

    fn parse_text(&self) -> Option<&str> {
        let end = self
            .data
            .iter()
            .position(|b| *b == b'\n' || *b == 0)
            .unwrap_or(self.data.len());
        let text = core::str::from_utf8(self.data.get(..end)?).ok()?.trim_end();
        (!text.is_empty()).then_some(text)
    }

    #[must_use]
    pub fn serial(&self) -> Option<&str> {
        if !matches!(self.tag, DescTag::SerialNumber) {
            return None;
        }
        self.parse_text()
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        if !matches!(self.tag, DescTag::MonitorName) {
            return None;
        }
        self.parse_text()
    }

    #[must_use]
    pub fn text(&self) -> Option<&str> {
        if !matches!(self.tag, DescTag::Text) {
            return None;
        }
        self.parse_text()
    }

    #[must_use]
    pub fn range(&self) -> Option<Range> {
        if !matches!(self.tag, DescTag::RangeLimits) {
            return None;
        }
        Range::parse(&self.raw_desc())
    }

    #[must_use]
    pub fn white_point(&self) -> Option<WhitePoint> {
        if !matches!(self.tag, DescTag::WhitePoint) {
            return None;
        }
        WhitePoint::parse(&self.raw_desc())
    }

    #[must_use]
    pub fn color(&self) -> Option<Color> {
        if !matches!(self.tag, DescTag::Dcm) {
            return None;
        }
        Color::parse(&self.raw_desc())
    }

    #[must_use]
    pub fn cvt3(&self) -> Option<Cvt3> {
        if !matches!(self.tag, DescTag::Cvt3Byte) {
            return None;
        }
        Cvt3::parse(&self.raw_desc())
    }

    #[must_use]
    pub fn std2(&self) -> Option<Std2> {
        if !matches!(self.tag, DescTag::StdTimings2) {
            return None;
        }
        Std2::parse(&self.raw_desc())
    }

    #[must_use]
    pub fn std3(&self) -> Option<Std3> {
        if !matches!(self.tag, DescTag::StdTiming3) {
            return None;
        }
        Std3::parse(&self.raw_desc())
    }

    #[must_use]
    pub const fn is_dummy(&self) -> bool {
        matches!(self.tag, DescTag::Dummy)
    }

    #[must_use]
    pub const fn vendor_tag(&self) -> Option<u8> {
        if matches!(self.tag, DescTag::VendorReserved) {
            Some(self.tag_raw)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn unknown_tag(&self) -> Option<u8> {
        if matches!(self.tag, DescTag::Unknown) {
            Some(self.tag_raw)
        } else {
            None
        }
    }
}
