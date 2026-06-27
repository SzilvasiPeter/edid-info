//! EDID Detailed Timing and Monitor Descriptors.
//!
//! Monitor descriptors provide additional display metadata
//! when bytes 0–1 of the 18-byte descriptor are both zero.
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
//! | 0xFA | Standard Timing | Additional standard timings |
//! | 0xF9 | DCM | Display color management |
//! | 0xF8 | CVT 3-Byte | CVT timing codes |
//! | 0xF7 | Established Timing III | Additional established timings |
//! | 0x00–0x0F | Vendor Reserved | Vendor-specific data |
//!
//! # Examples
//!
//! ```rust
//! use edid_info::base::descriptor::dtd::DetailedTiming;
//! use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};
//! use edid_info::common::DESC_LEN;
//!
//! // A detailed timing descriptor (non-zero pixel clock)
//! let mut dtd_raw = [0u8; DESC_LEN];
//! dtd_raw[0] = 0x01;
//! let timing = DetailedTiming::new(&dtd_raw);
//! assert_eq!(timing.pixel_clock_khz(), 10);
//!
//! // A monitor descriptor: product name (tag 0xFC)
//! let mut mon_raw = [0u8; DESC_LEN];
//! mon_raw[3] = 0xFC;
//! mon_raw[5..12].copy_from_slice(b"My Disp");
//! let monitor = Monitor::new(&mon_raw, false);
//! if let DisplayDescriptor::ProductName(name) = monitor.descriptor() {
//!     assert_eq!(name.text(), "My Disp");
//! }
//! ```
pub mod color;
pub mod cvt3;
pub mod dcm;
pub mod dtd;
pub mod established;
pub mod monitor;
pub mod range;
pub mod standard;
