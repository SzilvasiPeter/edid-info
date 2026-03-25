use edid_info::{base::footer::Footer, common::BLOCK_LEN};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_footer_acer_ek221q_h() {
    let raw: [u8; BLOCK_LEN] = std::array::from_fn(|i| ACER[i]);
    let out = Footer::new(&raw);

    assert_eq!(out.extension_num(), 1);
    assert_eq!(out.checksum(), 0x18);
}

#[test]
fn parse_footer_asus_rog_pg27u() {
    let raw: [u8; BLOCK_LEN] = std::array::from_fn(|i| ASUS[i]);
    let out = Footer::new(&raw);

    assert_eq!(out.extension_num(), 2);
    assert_eq!(out.checksum(), 0x72);
}
