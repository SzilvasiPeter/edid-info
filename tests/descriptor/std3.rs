use edid_info::edid::descriptor::std3::Std3;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_std3_not_present_acer_ek221q_h() {
    let raw: [u8; 18] = std::array::from_fn(|i| ACER[90 + i]);
    assert!(Std3::parse(&raw).is_none());
}

#[test]
fn parse_std3_not_present_asus_rog_pg27u() {
    let raw: [u8; 18] = std::array::from_fn(|i| ASUS[90 + i]);
    assert!(Std3::parse(&raw).is_none());
}

#[test]
fn parse_std3_synthetic() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF7;
    raw[5] = 0x10;
    raw[6] = 0x01;
    raw[7] = 0x02;
    raw[8] = 0x04;
    raw[9] = 0x08;
    raw[10] = 0x10;
    raw[11] = 0x20;

    let std3 = Std3::parse(&raw).expect("std3 parse");

    assert_eq!(std3.map(), [0x01, 0x02, 0x04, 0x08, 0x10, 0x20]);
    assert_eq!(std3.byte(0), Some(0x01));
    assert_eq!(std3.byte(1), Some(0x02));
    assert_eq!(std3.byte(2), Some(0x04));
    assert_eq!(std3.byte(3), Some(0x08));
    assert_eq!(std3.byte(4), Some(0x10));
    assert_eq!(std3.byte(5), Some(0x20));
    assert_eq!(std3.byte(6), None);

    assert!(std3.has(0, 0).unwrap());
    assert!(!std3.has(0, 1).unwrap());
    assert!(std3.has(1, 1).unwrap());
    assert!(std3.has(2, 2).unwrap());
    assert!(std3.has(3, 3).unwrap());
    assert!(std3.has(4, 4).unwrap());
    assert!(std3.has(5, 5).unwrap());
}

#[test]
fn parse_std3_wrong_version() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF7;
    raw[5] = 0x09;

    assert!(Std3::parse(&raw).is_none());
}

#[test]
fn parse_std3_non_zero_padding() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF7;
    raw[5] = 0x10;
    raw[12] = 0x01;

    assert!(Std3::parse(&raw).is_none());
}
