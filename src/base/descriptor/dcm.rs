//! Display Color Management (DCM) descriptor (tag 0xF9).
//!
//! Stores color correction polynomial coefficients for the display panel.
//!
//! # Descriptor Layout
//!
//! | Byte | Content |
//! |------|---------|
//! | 0–4  | `0x0000F900` (tag header) |
//! | 5    | Version (`0x03`) |
//! | 6–7  | Red a3 |
//! | 8–9  | Red a2 |
//! | 10–11| Green a3 |
//! | 12–13| Green a2 |
//! | 14–15| Blue a3 |
//! | 16–17| Blue a2 |
//!
//! Each coefficient is a 16-bit unsigned integer stored LSB-first.
//!
//! # Examples
//!
//! ```rust
//! use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};
//! use edid_info::common::DESC_LEN;
//!
//! let mut raw = [0u8; DESC_LEN];
//! raw[3] = 0xF9; // DCM tag
//! raw[5] = 0x03; // Version
//!
//! let monitor = Monitor::new(&raw, false);
//! if let DisplayDescriptor::Dcm(dcm) = monitor.descriptor() {
//!     assert_eq!(dcm.red_a3(), 0);
//!     assert_eq!(dcm.green_a3(), 0);
//!     assert_eq!(dcm.blue_a3(), 0);
//!     assert!(dcm.validate().is_valid());
//! } else {
//!     panic!("expected DCM descriptor");
//! }
//! ```

use crate::common::{Validation, WarningKind};

/// Display Color Management polynomial coefficients.
///
/// Stores the raw 13-byte descriptor payload and lazily computes
/// each 16-bit coefficient on access.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dcm {
    raw: [u8; 13],
}

impl Dcm {
    pub(super) const fn new(raw: &[u8; 13]) -> Self {
        Self { raw: *raw }
    }

    /// Red a3 coefficient.
    #[must_use]
    pub const fn red_a3(&self) -> u16 {
        u16::from_le_bytes([self.raw[1], self.raw[2]])
    }

    /// Red a2 coefficient.
    #[must_use]
    pub const fn red_a2(&self) -> u16 {
        u16::from_le_bytes([self.raw[3], self.raw[4]])
    }

    /// Green a3 coefficient.
    #[must_use]
    pub const fn green_a3(&self) -> u16 {
        u16::from_le_bytes([self.raw[5], self.raw[6]])
    }

    /// Green a2 coefficient.
    #[must_use]
    pub const fn green_a2(&self) -> u16 {
        u16::from_le_bytes([self.raw[7], self.raw[8]])
    }

    /// Blue a3 coefficient.
    #[must_use]
    pub const fn blue_a3(&self) -> u16 {
        u16::from_le_bytes([self.raw[9], self.raw[10]])
    }

    /// Blue a2 coefficient.
    #[must_use]
    pub const fn blue_a2(&self) -> u16 {
        u16::from_le_bytes([self.raw[11], self.raw[12]])
    }

    /// Validates the DCM descriptor.
    ///
    /// **Warnings**: version byte is not `0x03`.
    #[must_use]
    pub const fn validate(&self) -> Validation {
        Validation::new().warn_if(self.raw[0] != 0x03, WarningKind::DcmVersionReserved)
    }
}

#[cfg(test)]
mod tests {
    use crate::base::descriptor::monitor::{DisplayDescriptor, Monitor};
    use crate::common::DESC_LEN;

    #[test]
    fn examples_docstring() {
        let mut raw = [0u8; DESC_LEN];
        raw[3] = 0xF9; // DCM tag
        raw[5] = 0x03; // Version

        let monitor = Monitor::new(&raw, false);
        if let DisplayDescriptor::Dcm(dcm) = monitor.descriptor() {
            assert_eq!(dcm.red_a3(), 0);
            assert_eq!(dcm.green_a3(), 0);
            assert_eq!(dcm.blue_a3(), 0);
            assert!(dcm.validate().is_valid());
        } else {
            panic!("expected DCM descriptor");
        }
    }
}
