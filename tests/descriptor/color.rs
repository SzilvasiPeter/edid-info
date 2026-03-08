use edid_info::edid::descriptor::color::Color;

const ACER: &[u8] = include_bytes!("../data/acer_ek221q_h.edid");
const ASUS: &[u8] = include_bytes!("../data/asus_rog_pg27u.edid");

#[test]
fn parse_color_not_present_acer_ek221q_h() {
    let raw: &[u8; 18] = ACER[90..108].try_into().expect("bytes");
    assert!(Color::parse(raw).is_none());
}

#[test]
fn parse_color_not_present_asus_rog_pg27u() {
    let raw: &[u8; 18] = ASUS[90..108].try_into().expect("bytes");
    assert!(Color::parse(raw).is_none());
}

#[test]
fn parse_color_synthetic() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF9;
    raw[5] = 0x03;
    raw[6] = 0x10;
    raw[7] = 0x00;
    raw[8] = 0x20;
    raw[9] = 0x00;
    raw[10] = 0x30;
    raw[11] = 0x00;
    raw[12] = 0x40;
    raw[13] = 0x00;
    raw[14] = 0x50;
    raw[15] = 0x00;
    raw[16] = 0x60;
    raw[17] = 0x00;

    let color = Color::parse(&raw).expect("color parse");
    assert_eq!(color.red_a3(), 0x0010);
    assert_eq!(color.red_a2(), 0x0020);
    assert_eq!(color.green_a3(), 0x0030);
    assert_eq!(color.green_a2(), 0x0040);
    assert_eq!(color.blue_a3(), 0x0050);
    assert_eq!(color.blue_a2(), 0x0060);
}

#[test]
fn parse_color_wrong_version() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF9;
    raw[5] = 0x02;

    assert!(Color::parse(&raw).is_none());
}
