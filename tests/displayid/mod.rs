mod block;
mod interface;
mod params;
mod product;
mod timing;

use edid_info::edid::BLOCK_LEN;
use edid_info::edid::displayid::DisplayId;

const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn test_displayid_parse_asus() {
    // Asus block 3 starts at 256
    let raw: &[u8; BLOCK_LEN] = ASUS[256..384].try_into().unwrap();
    let did = DisplayId::parse(raw).expect("should parse Asus DisplayID block");

    // Based on our inspection:
    // raw[0] = 70 (Tag)
    // raw[1] = 13 (Revision)
    // raw[2] = 79 (Version 121?)
    assert_eq!(did.version(), 121);
    assert_eq!(did.section_len(), 3);
    assert!(did.checksum_ok());
}

#[test]
fn test_displayid_parse_invalid_tag() {
    let mut raw = [0u8; BLOCK_LEN];
    raw[0] = 0x00; // Not 0x70
    assert!(DisplayId::parse(&raw).is_none());
}

#[test]
fn test_displayid_parse_zero_version() {
    let mut raw = [0u8; BLOCK_LEN];
    raw[0] = 0x70;
    raw[2] = 0x00; // Zero version
    assert!(DisplayId::parse(&raw).is_none());
}
