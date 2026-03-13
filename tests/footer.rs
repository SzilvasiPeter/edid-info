use edid_info::edid::footer::{FOOTER_LEN, FOOTER_OFF, Footer};

const ACER: &[u8] = include_bytes!("data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_footer_acer_ek221q_h() {
    let raw: [u8; FOOTER_LEN] = std::array::from_fn(|i| ACER[FOOTER_OFF + i]);
    let out = Footer::parse(&raw);

    assert_eq!(out.extension_num(), 1);
    assert_eq!(out.checksum(), 0x18);
}

#[test]
fn parse_footer_asus_rog_pg27u() {
    let raw: [u8; FOOTER_LEN] = std::array::from_fn(|i| ASUS[FOOTER_OFF + i]);
    let out = Footer::parse(&raw);

    assert_eq!(out.extension_num(), 2);
    assert_eq!(out.checksum(), 0x72);
}
