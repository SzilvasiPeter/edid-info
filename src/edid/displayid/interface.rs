//! `DisplayID` Interface Features Data Block.
//!
//! Describes the display interface capabilities including bits per
//! component (BPC) support for various color formats and supported
//! electro-optical transfer functions (EOTFs).
//!
//! # Structure
//!
//! | Byte | Description |
//! |------|-------------|
//! | 0    | RGB BPC support |
//! | 1    | YCbCr 4:4:4 BPC support |
//! | 2    | YCbCr 4:2:2 BPC support |
//! | 3    | YCbCr 4:2:0 BPC support |
//! | 6    | EOTF support flags (sRGB, BT.2020, ST.2084) |

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InterfaceFeatures<'a> {
    data: &'a [u8],
}

impl<'a> InterfaceFeatures<'a> {
    #[must_use]
    pub const fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < 9 {
            return None;
        }
        Some(Self { data })
    }

    #[must_use]
    pub const fn bpc_rgb(&self) -> u8 {
        self.data[0]
    }
    #[must_use]
    pub const fn bpc_y444(&self) -> u8 {
        self.data[1]
    }
    #[must_use]
    pub const fn bpc_y422(&self) -> u8 {
        self.data[2]
    }
    #[must_use]
    pub const fn bpc_y420(&self) -> u8 {
        self.data[3]
    }

    #[must_use]
    pub fn srgb_eotf(&self) -> bool {
        (self.data[6] & 0x01) != 0
    }
    #[must_use]
    pub fn bt2020_eotf(&self) -> bool {
        (self.data[6] & 0x20) != 0
    }
    #[must_use]
    pub fn bt2020_st2084_eotf(&self) -> bool {
        (self.data[6] & 0x40) != 0
    }
}
