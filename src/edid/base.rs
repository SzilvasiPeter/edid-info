use crate::edid::BLOCK_LEN;
use crate::edid::basic::Basic;
use crate::edid::chroma::Chroma;
use crate::edid::dtd::Dtd;
use crate::edid::established::Established;
use crate::edid::footer::Footer;
use crate::edid::header::Header;
use crate::edid::std1::Std1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BaseEdid {
    raw: [u8; BLOCK_LEN],
    header: Header,
    basic: Basic,
    chroma: Chroma,
    established: Established,
    standard: Std1,
    dtd: Dtd,
    footer: Footer,
}

impl BaseEdid {
    #[must_use]
    pub fn parse(raw: &[u8; BLOCK_LEN]) -> Self {
        Self {
            raw: *raw,
            header: Header::parse(raw),
            basic: Basic::parse_base(raw),
            chroma: Chroma::parse_base(raw),
            established: Established::parse_base(raw),
            standard: Std1::parse_base(raw),
            dtd: Dtd::parse_base(raw),
            footer: Footer::parse(raw),
        }
    }

    #[must_use]
    pub const fn header(&self) -> Header {
        self.header
    }

    #[must_use]
    pub const fn basic(&self) -> Basic {
        self.basic
    }

    #[must_use]
    pub const fn chroma(&self) -> Chroma {
        self.chroma
    }

    #[must_use]
    pub const fn established(&self) -> Established {
        self.established
    }

    #[must_use]
    pub const fn standard(&self) -> Std1 {
        self.standard
    }

    #[must_use]
    pub const fn dtd(&self) -> Dtd {
        self.dtd
    }

    #[must_use]
    pub const fn footer(&self) -> Footer {
        self.footer
    }

    #[must_use]
    pub fn checksum_ok(&self) -> bool {
        Footer::checksum_ok(&self.raw)
    }
}
