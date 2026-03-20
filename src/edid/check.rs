//! EDID block checksum verification.
//!
//! EDID blocks use a simple checksum: the sum of all bytes in a block
//! must equal zero (with wraparound).

use crate::edid::BLOCK_LEN;

/// Verifies the checksum of an EDID block.
///
/// Returns `true` if the sum of all bytes in the block equals zero (with u8 wrapping).
#[must_use]
pub const fn checksum_ok(raw: &[u8; BLOCK_LEN]) -> bool {
    let mut sum = 0u8;
    let mut i = 0;
    while i < BLOCK_LEN {
        sum = sum.wrapping_add(raw[i]);
        i += 1;
    }
    sum == 0
}
