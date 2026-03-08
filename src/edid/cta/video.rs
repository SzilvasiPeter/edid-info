use crate::edid::cta::vic::Vic;

/// Short Video Descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Svd {
    vic: u8,
    native: bool,
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

    #[must_use]
    pub const fn parse(raw: u8) -> Self {
        let lo = raw & 0b0111_1111;
        let hi = (raw & 0b1000_0000) != 0;
        let native = hi && lo > 0 && lo <= 64;
        let vic = if hi && lo >= 65 { lo | 0b1000_0000 } else { lo };
        Self { vic, native }
    }
}
