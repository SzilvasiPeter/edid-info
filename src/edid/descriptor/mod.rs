//! EDID Detailed Timing and Monitor Descriptors.
//!
//! Monitor descriptors (also called "blanking descriptors") provide
//! additional display metadata when bytes 0–1 of the 18-byte descriptor
//! are both zero.
//!
//! # Descriptor Types (byte 3)
//!
//! | Tag | Name | Description |
//! |-----|------|-------------|
//! | 0xFF | Serial Number | ASCII serial number |
//! | 0xFE | Text | ASCII text string |
//! | 0xFD | Range Limits | Vertical/horizontal ranges |
//! | 0xFC | Monitor Name | ASCII monitor name |
//! | 0xFB | White Point | White point data |
//! | 0xFA | Standard Timing 2 | Additional standard timings |
//! | 0xF9 | DCM | Display color management |
//! | 0xF8 | CVT 3-Byte | CVT timing codes |
//! | 0xF7 | Standard Timing 3 | More standard timings |
//! | 0x00–0x0F | Vendor Reserved | Vendor-specific data |

/// Length of a detailed timing or monitor descriptor in bytes.
pub const DESC_LEN: usize = 18;

pub mod color;
pub mod cvt3;
pub mod monitor;
pub mod range;
pub mod std2;
pub mod std3;
pub mod timing;
pub mod white_point;
