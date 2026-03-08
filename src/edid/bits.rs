//! Bit manipulation helpers for EDID parsing.

/// Checks if a specific bit is set in a byte.
#[must_use]
pub const fn is_set(val: u8, bit: u8) -> bool {
    (val & (1 << bit)) != 0
}

/// Extracts bits from a byte using a mask and shifting right.
#[must_use]
pub const fn get_bits(val: u8, mask: u8, shift: u8) -> u8 {
    (val & mask) >> shift
}

/// Packs a 12-bit value where the lower 8 bits are in one byte and the upper 4 bits
/// are in the high nibble of another byte.
#[must_use]
pub const fn u12_hi(lo: u8, mix: u8) -> u16 {
    u16::from_le_bytes([lo, (mix >> 4) & 0x0f])
}

/// Packs a 12-bit value where the lower 8 bits are in one byte and the upper 4 bits
/// are in the low nibble of another byte.
#[must_use]
pub const fn u12_lo(lo: u8, mix: u8) -> u16 {
    u16::from_le_bytes([lo, mix & 0x0f])
}

/// Packs a 10-bit value where the lower 8 bits are in one byte and the upper 2 bits
/// are provided as a u16 (usually extracted via bitfield).
#[must_use]
pub const fn u10_lo(lo: u8, hi: u16) -> u16 {
    u16::from_le_bytes([lo, (hi & 0x03) as u8])
}

/// Packs a 10-bit value where the upper 8 bits are in one byte and the lower 2 bits
/// are provided as a u16 (usually extracted via bitfield).
#[must_use]
pub const fn u10_hi(msb: u8, lsb: u16) -> u16 {
    ((msb as u16) << 2) | (lsb & 0x03)
}

/// Extracts a 2-bit value from a byte using two bit masks (high and low bits).
#[must_use]
pub const fn u2_from_masks(value: u8, hi_mask: u8, lo_mask: u8) -> u16 {
    let hi = if (value & hi_mask) != 0 { 2 } else { 0 };
    let lo = if (value & lo_mask) != 0 { 1 } else { 0 };
    hi | lo
}

/// Packs a 6-bit value from two parts.
#[must_use]
pub const fn u6_pack(lo4: u8, hi2: u8) -> u8 {
    (lo4 & 0x0f) | ((hi2 & 0x03) << 4)
}
