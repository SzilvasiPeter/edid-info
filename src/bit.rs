//! Bit manipulation helpers for EDID parsing.
//!
//! EDID stores many fields as bitfields packed across bytes.

/// Checks if a specific bit is set in a byte.
#[must_use]
pub const fn is_set(byte: u8, bit: u8) -> bool {
    debug_assert!(bit < 8, "bit has to be less than 8");
    (byte >> bit) & 0b0000_0001 != 0
}

/// Extracts bits from a byte given a start bit (inclusive) and end bit (inclusive).
/// Bits are numbered from 0 (LSB) to 7 (MSB).
#[must_use]
pub const fn get_bits(byte: u8, start: u8, end: u8) -> u8 {
    debug_assert!(start <= end && end <= 7, "invalid bit range");
    let width = end - start;
    (byte >> start) & (0b1111_1111 >> (7 - width))
}

/// Combines two 8-bit values into a `u16` by shifting `hi` left by `lo_width` and performing a bitwise OR with `lo`.
#[must_use]
pub const fn pack_bits(hi: u8, lo: u8, lo_width: u8) -> u16 {
    debug_assert!(lo_width <= 8, "lo_width must be <= 8");
    ((hi as u16) << lo_width) | (lo as u16)
}
