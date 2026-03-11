//! EDID (Extended Display Identification Data) parsing library.
//!
//! EDID is a VESA-standardized metadata format that describes a display's
//! capabilities and parameters to a video source. This crate provides
//! zero-copy parsing of EDID 1.4 base blocks and common extensions
//! (CTA-861 2.0).
//!
//! # Structure
//!
//! An EDID blob consists of:
//! - A 128-byte base block (EDID 1.4) containing manufacturer info, basic
//!   display parameters, color characteristics, standard timings, and
//!   detailed timing descriptors.
//! - Zero or more 128-byte extension blocks (CTA-861, etc.).
//!
//! # References
//!
//! - [EDID 1.4 Specification](https://grouper.ieee.org/groups/2040/2040_14.pdf)
//! - [Wikipedia: EDID Structure, version 1.4](https://en.wikipedia.org/wiki/Extended_Display_Identification_Data#Structure,_version_1.4)

#![forbid(unsafe_code)]
pub mod edid;
