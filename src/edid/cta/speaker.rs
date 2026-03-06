#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Speaker {
    FlFr,
    Lfe,
    Fc,
    BlBr,
    Bc,
    FlcFrc,
    RlcRrc,
    FlwFrw,
    TpflTpfr,
    Tpc,
    Tpfc,
    LsRs,
    Lfe2,
    Tpbc,
    SilSir,
    TpsilTpsir,
    TpblTpbr,
    Btfc,
    BtflBtfr,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpeakerAlloc {
    b1: u8,
    b2: u8,
    b3: u8,
}

impl SpeakerAlloc {
    #[must_use]
    pub const fn parse(b1: u8, b2: u8, b3: u8) -> Self {
        Self { b1, b2, b3 }
    }

    #[must_use]
    pub const fn bytes(&self) -> (u8, u8, u8) {
        (self.b1, self.b2, self.b3)
    }

    #[must_use]
    pub const fn has(&self, spk: Speaker) -> bool {
        let (b, m) = match spk {
            Speaker::FlFr => (self.b1, 0b0000_0001),
            Speaker::Lfe => (self.b1, 0b0000_0010),
            Speaker::Fc => (self.b1, 0b0000_0100),
            Speaker::BlBr => (self.b1, 0b0000_1000),
            Speaker::Bc => (self.b1, 0b0001_0000),
            Speaker::FlcFrc => (self.b1, 0b0010_0000),
            Speaker::RlcRrc => (self.b1, 0b0100_0000),
            Speaker::FlwFrw => (self.b1, 0b1000_0000),
            Speaker::TpflTpfr => (self.b2, 0b0000_0001),
            Speaker::Tpc => (self.b2, 0b0000_0010),
            Speaker::Tpfc => (self.b2, 0b0000_0100),
            Speaker::LsRs => (self.b2, 0b0000_1000),
            Speaker::Lfe2 => (self.b2, 0b0001_0000),
            Speaker::Tpbc => (self.b2, 0b0010_0000),
            Speaker::SilSir => (self.b2, 0b0100_0000),
            Speaker::TpsilTpsir => (self.b2, 0b1000_0000),
            Speaker::TpblTpbr => (self.b3, 0b0000_0001),
            Speaker::Btfc => (self.b3, 0b0000_0010),
            Speaker::BtflBtfr => (self.b3, 0b0000_0100),
        };
        (b & m) != 0
    }
}
