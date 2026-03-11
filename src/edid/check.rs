//! EDID block checksum verification.
//!
//! EDID blocks use a simple checksum: the sum of all bytes in a block
//! must equal zero (with wraparound).

/// Verifies the checksum of an EDID block.
///
/// Returns `true` if the sum of all bytes in the block equals zero
/// (with u8 wrapping).
#[must_use]
pub fn check(raw: &[u8]) -> bool {
    raw.iter().fold(0u8, |a, b| a.wrapping_add(*b)) == 0
}
