use edid_info::edid::descriptor::cvt3::{Aspect, Cvt3, PrefRate};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_cvt3_not_present_acer_ek221q_h() {
    let raw: &[u8; 18] = ACER[90..108].try_into().expect("bytes");
    assert!(Cvt3::parse(raw).is_none());
}

#[test]
fn parse_cvt3_not_present_asus_rog_pg27u() {
    let raw: &[u8; 18] = ASUS[90..108].try_into().expect("bytes");
    assert!(Cvt3::parse(raw).is_none());
}

#[test]
fn parse_cvt3_synthetic() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF8;
    raw[5] = 0x01;
    raw[6] = 0x00;
    raw[7] = 0x00;
    raw[8] = 0b0000_0000;
    raw[9] = 0x10;
    raw[10] = 0x04;
    raw[11] = 0b0011_0000;
    raw[12] = 0x20;
    raw[13] = 0x08;
    raw[14] = 0b0101_0000;
    raw[15] = 0x30;
    raw[16] = 0x0C;
    raw[17] = 0b0111_0000;

    let cvt3 = Cvt3::parse(&raw).expect("cvt3 parse");

    let mode1 = cvt3.mode1();
    assert_eq!(mode1.addr_lines(), 0);
    assert_eq!(mode1.aspect(), Aspect::A4_3);
    assert_eq!(mode1.pref(), PrefRate::Hz50);
    assert!(!mode1.hz50());
    assert!(!mode1.hz60());
    assert!(!mode1.hz75());
    assert!(!mode1.hz85());
    assert!(!mode1.hz60_rb());

    let mode2 = cvt3.mode2();
    assert_eq!(mode2.addr_lines(), 16);
    assert_eq!(mode2.aspect(), Aspect::A16_9);
    assert_eq!(mode2.pref(), PrefRate::Hz60);

    let mode3 = cvt3.mode3();
    assert_eq!(mode3.addr_lines(), 32);
    assert_eq!(mode3.aspect(), Aspect::A16_10);
    assert_eq!(mode3.pref(), PrefRate::Hz75);

    let mode4 = cvt3.mode4();
    assert_eq!(mode4.addr_lines(), 48);
    assert_eq!(mode4.aspect(), Aspect::A15_9);
    assert_eq!(mode4.pref(), PrefRate::Hz85);
}

#[test]
fn parse_cvt3_wrong_version() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF8;
    raw[5] = 0x02;

    assert!(Cvt3::parse(&raw).is_none());
}

#[test]
fn parse_mode_v_lines() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF8;
    raw[5] = 0x01;
    raw[6] = 0x09;
    raw[7] = 0x00;
    raw[8] = 0b0000_0000;

    let cvt3 = Cvt3::parse(&raw).expect("cvt3 parse");
    let mode1 = cvt3.mode1();
    assert_eq!(mode1.addr_lines(), 9);
    assert_eq!(mode1.v_lines(), 20);
}

#[test]
fn parse_mode_h_pixels() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF8;
    raw[5] = 0x01;
    raw[6] = 0x09;
    raw[7] = 0x00;
    raw[8] = 0b0000_0000;

    let cvt3 = Cvt3::parse(&raw).expect("cvt3 parse");
    let mode1 = cvt3.mode1();
    assert_eq!(mode1.h_pixels(), 24);
}
