use edid_info::base::descriptor::cvt3::Rate;
use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};
use edid_info::base::dmt::find_cvt;
use edid_info::common::AspectRatio;

const ACER: &[u8] = include_bytes!("../../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_cvt3_not_present_acer_ek221q_h() {
    let raw: [u8; 18] = std::array::from_fn(|i| ACER[90 + i]);
    let monitor = Monitor::new(&raw, false);
    assert!(!matches!(monitor.descriptor(), DisplayDescriptor::Cvt3(_)));
}

#[test]
fn parse_cvt3_not_present_asus_rog_pg27u() {
    let raw: [u8; 18] = std::array::from_fn(|i| ASUS[90 + i]);
    let monitor = Monitor::new(&raw, false);
    assert!(!matches!(monitor.descriptor(), DisplayDescriptor::Cvt3(_)));
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

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::Cvt3(cvt3) = monitor.descriptor() {
        let p1 = cvt3.priority1();
        assert_eq!(p1.vertical_lines(), 2);
        assert_eq!(p1.aspect_ratio(), AspectRatio::new(4, 3));
        assert_eq!(p1.preferred_rate(), Rate::Hz50);
        assert_eq!(p1.rates().count(), 0);
        assert!(!p1.blanking().standard);
        assert!(!p1.blanking().reduced);

        let p2 = cvt3.priority2();
        assert_eq!(p2.vertical_lines(), 34);
        assert_eq!(p2.aspect_ratio(), AspectRatio::new(16, 9));
        assert_eq!(p2.preferred_rate(), Rate::Hz60);
        assert!(p2.rates().eq([Rate::Hz50]));
        assert!(p2.blanking().standard);
        assert!(!p2.blanking().reduced);

        let p3 = cvt3.priority3();
        assert_eq!(p3.vertical_lines(), 66);
        assert_eq!(p3.aspect_ratio(), AspectRatio::new(16, 10));
        assert_eq!(p3.preferred_rate(), Rate::Hz75);

        let p4 = cvt3.priority4();
        assert_eq!(p4.vertical_lines(), 98);
        assert_eq!(p4.aspect_ratio(), AspectRatio::new(15, 9));
        assert_eq!(p4.preferred_rate(), Rate::Hz85);
    } else {
        panic!("expected Cvt3, got {:?}", monitor.descriptor());
    }
}

#[test]
fn parse_mode_vertical_lines() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF8;
    raw[5] = 0x01;
    raw[6] = 0x09;
    raw[7] = 0x00;
    raw[8] = 0b0000_0000;

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::Cvt3(cvt3) = monitor.descriptor() {
        let p1 = cvt3.priority1();
        assert_eq!(p1.vertical_lines(), 20);
    } else {
        panic!("expected Cvt3, got {:?}", monitor.descriptor());
    }
}

#[test]
fn parse_mode_horizontal_pixels() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF8;
    raw[5] = 0x01;
    raw[6] = 0x09;
    raw[7] = 0x00;
    raw[8] = 0b0000_0000;

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::Cvt3(cvt3) = monitor.descriptor() {
        let p1 = cvt3.priority1();
        assert_eq!(p1.horizontal_pixels(), 24);
    } else {
        panic!("expected Cvt3, got {:?}", monitor.descriptor());
    }
}

#[test]
fn find_cvt_entries() {
    let expected = [
        (0x7F_1C21u32, 0x16),
        (0x7F_1C28, 0x17),
        (0x7F_1C44, 0x18),
        (0x7F_1C62, 0x19),
        (0x8F_1821, 0x1B),
        (0x8F_1828, 0x1C),
        (0x8F_1844, 0x1D),
        (0x8F_1862, 0x1E),
        (0x0C_2021, 0x29),
        (0xC1_1821, 0x2E),
        (0x57_2821, 0x44),
        (0x1F_3821, 0x4C),
    ];

    for (cvt_code, expected_id) in expected {
        let dmt = find_cvt(cvt_code).unwrap();
        assert_eq!(dmt.id, expected_id);
        assert_eq!(dmt.cvt_code, Some(cvt_code));
    }

    assert_eq!(find_cvt(0xDEAD_BEEF), None);
}
