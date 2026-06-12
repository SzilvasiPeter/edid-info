use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};

const ACER: &[u8] = include_bytes!("../../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_color_not_present_acer_ek221q_h() {
    let raw: [u8; 18] = std::array::from_fn(|i| ACER[90 + i]);
    let monitor = Monitor::new(&raw, false);
    assert!(!matches!(monitor.descriptor(), DisplayDescriptor::Dcm(_)));
}

#[test]
fn parse_color_not_present_asus_rog_pg27u() {
    let raw: [u8; 18] = std::array::from_fn(|i| ASUS[90 + i]);
    let monitor = Monitor::new(&raw, false);
    assert!(!matches!(monitor.descriptor(), DisplayDescriptor::Dcm(_)));
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

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::Dcm(color) = monitor.descriptor() {
        assert_eq!(color.red_a3(), 0x0010);
        assert_eq!(color.red_a2(), 0x0020);
        assert_eq!(color.green_a3(), 0x0030);
        assert_eq!(color.green_a2(), 0x0040);
        assert_eq!(color.blue_a3(), 0x0050);
        assert_eq!(color.blue_a2(), 0x0060);
    } else {
        panic!("expected DCM, got {:?}", monitor.descriptor());
    }
}
