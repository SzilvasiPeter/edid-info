//! Standard Timing Identification Level 2.
//!
//! Additional standard timing descriptors using the same format
//! as the base Standard Timing Identification block. Uses tag 0xFA.
//! Contains 6 timing entries (12 bytes total).

use crate::base::std1::{StdTiming, parse_std};
use crate::common::DESC_LEN;

const MODE_NUM: usize = 6;
const MODE_LEN: usize = MODE_NUM * 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Std2 {
    bytes: [u8; MODE_LEN],
    legacy: bool,
}

impl Std2 {
    #[must_use]
    pub(super) fn parse(raw: &[u8; DESC_LEN], legacy: bool) -> Self {
        let mut bytes = [0u8; MODE_LEN];
        bytes.copy_from_slice(&raw[5..17]);
        Self { bytes, legacy }
    }

    pub fn modes(&self) -> impl Iterator<Item = StdTiming> {
        self.bytes
            .chunks_exact(2)
            .filter_map(move |c| parse_std(c[0], c[1], self.legacy))
    }
}
