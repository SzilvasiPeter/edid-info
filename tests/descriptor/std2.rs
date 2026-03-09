use edid_info::edid::descriptor::std2::Std2;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_std2_not_present_acer_ek221q_h() {
    let raw: &[u8; 18] = ACER[90..108].try_into().expect("bytes");
    assert!(Std2::parse(raw).is_none());
}

#[test]
fn parse_std2_not_present_asus_rog_pg27u() {
    let raw: &[u8; 18] = ASUS[90..108].try_into().expect("bytes");
    assert!(Std2::parse(raw).is_none());
}

#[test]
fn parse_std2_synthetic() {
    let mut raw = [0u8; 18];
    raw[3] = 0xFA;
    raw[5] = 0x40;
    raw[6] = 0x01;
    raw[7] = 0x41;
    raw[8] = 0x02;
    raw[9] = 0x42;
    raw[10] = 0x03;
    raw[11] = 0x43;
    raw[12] = 0x04;
    raw[13] = 0x44;
    raw[14] = 0x05;
    raw[15] = 0x45;
    raw[16] = 0x06;
    raw[17] = 0xFF;

    let std2 = Std2::parse(&raw).expect("std2 parse");

    assert!(std2.mode(0).is_some());
    assert!(std2.mode(1).is_some());
    assert!(std2.mode(2).is_some());
    assert!(std2.mode(3).is_some());
    assert!(std2.mode(4).is_some());
    assert!(std2.mode(5).is_some());
    assert!(std2.mode(6).is_none());

    assert_eq!(std2.pad(), 0xFF);
}

#[test]
fn parse_std2_empty_modes() {
    let mut raw = [0u8; 18];
    raw[3] = 0xFA;
    raw[5] = 0x01;
    raw[6] = 0x01;

    let std2 = Std2::parse(&raw).expect("std2 parse");

    assert!(std2.mode(0).is_none());
    assert!(std2.mode(1).is_some());
    assert!(std2.mode(2).is_some());
    assert!(std2.mode(3).is_some());
    assert!(std2.mode(4).is_some());
    assert!(std2.mode(5).is_some());
    assert_eq!(std2.pad(), 0);
}
