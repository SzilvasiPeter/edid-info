//! Standard Timing Identification Level 2.
//!
//! Additional standard timing descriptors using the same format
//! as the base Standard Timing Identification block. Uses tag 0xFA.
//! Contains 6 timing entries (12 bytes total).

use crate::base::standard::{StandardTiming, parse_std};

const MODE_NUM: usize = 6;
const MODE_LEN: usize = MODE_NUM * 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdTimings {
    bytes: [u8; MODE_LEN],
    legacy: bool,
}

impl StdTimings {
    pub(super) fn new(raw: &[u8; 13], legacy: bool) -> Self {
        let bytes: [u8; MODE_LEN] = raw[..12].try_into().map_or([0; MODE_LEN], |arr| arr);
        Self { bytes, legacy }
    }

    pub fn iter(&self) -> impl Iterator<Item = StandardTiming> {
        self.bytes
            .chunks_exact(2)
            .filter_map(move |c| parse_std(c[0], c[1], self.legacy))
    }
}
