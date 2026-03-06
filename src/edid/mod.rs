pub mod base;
pub mod basic;
pub mod chroma;
pub mod cta;
pub mod descriptor;
pub mod dtd;
pub mod established;
pub mod footer;
pub mod header;
pub mod std1;

#[must_use]
pub fn check(raw: &[u8]) -> bool {
    raw.iter().fold(0u8, |a, b| a.wrapping_add(*b)) == 0
}
