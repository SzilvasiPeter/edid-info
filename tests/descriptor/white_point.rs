use edid_info::edid::descriptor::white_point::WhitePoint;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_white_point_not_present_acer_ek221q_h() {
    let raw: &[u8; 18] = ACER[90..108].try_into().expect("bytes");
    assert!(WhitePoint::parse(raw).is_none());
}

#[test]
fn parse_white_point_not_present_asus_rog_pg27u() {
    let raw: &[u8; 18] = ASUS[90..108].try_into().expect("bytes");
    assert!(WhitePoint::parse(raw).is_none());
}

#[test]
fn parse_white_point_synthetic() {
    let mut raw = [0u8; 18];
    raw[3] = 0xFB;
    raw[5] = 0x01;
    raw[6] = 0b0000_0000;
    raw[7] = 0x80;
    raw[8] = 0x00;
    raw[9] = 0x40;
    raw[10] = 0x02;
    raw[11] = 0b0000_0101;
    raw[12] = 0x40;
    raw[13] = 0x00;
    raw[14] = 0x50;
    raw[15] = 0x00;
    raw[16] = 0x00;
    raw[17] = 0x00;

    let wp = WhitePoint::parse(&raw).expect("white point parse");

    let first = wp.first().expect("first point");
    assert_eq!(first.index(), 1);
    assert_eq!(first.x_raw(), 0x200);
    assert_eq!(first.y_raw(), 0x000);
    assert_eq!(first.gamma_raw(), 0x40);
    assert!((first.gamma() - 1.64).abs() < 0.01);

    let second = wp.second().expect("second point");
    assert_eq!(second.index(), 2);
    assert_eq!(second.x_raw(), 0x101);
    assert_eq!(second.y_raw(), 1);
    assert_eq!(second.gamma_raw(), 0x50);
    assert!((second.gamma() - 1.80).abs() < 0.01);

    assert_eq!(wp.pad(), [0x00, 0x00, 0x00]);
}

#[test]
fn parse_white_point_single_point() {
    let mut raw = [0u8; 18];
    raw[3] = 0xFB;
    raw[5] = 0x01;
    raw[6] = 0b0000_0000;
    raw[7] = 0x80;
    raw[8] = 0x00;
    raw[9] = 0x40;

    let wp = WhitePoint::parse(&raw).expect("white point parse");

    assert!(wp.first().is_some());
    assert!(wp.second().is_none());
}

#[test]
fn parse_white_point_gamma() {
    let mut raw = [0u8; 18];
    raw[3] = 0xFB;
    raw[5] = 0x01;
    raw[6] = 0b0000_0000;
    raw[7] = 0x80;
    raw[8] = 0x00;
    raw[9] = 0x64;

    let wp = WhitePoint::parse(&raw).expect("white point parse");
    let first = wp.first().expect("first point");
    assert_eq!(first.gamma_raw(), 0x64);
    assert!((first.gamma() - 2.0).abs() < 0.01);
}
