use edid_info::edid::footer::Footer;

const EDID: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_footer_acer_ek221q_h() {
    let raw: &[u8; 128] = EDID[0..128].try_into().expect("base block bytes");
    let out = Footer::parse(raw);

    assert_eq!(out.extension_num(), 1);
    assert_eq!(out.checksum(), 0x18);
    assert!(Footer::checksum_ok(raw));
}
