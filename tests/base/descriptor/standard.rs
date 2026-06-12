use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};

// TODO: use real world example instead of synthetic
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

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::StdTimings(std2) = monitor.descriptor() {
        assert_eq!(std2.iter().count(), 6);
    } else {
        panic!("expected StdTimings2, got {:?}", monitor.descriptor());
    }
}
#[test]
fn parse_std2_empty_modes() {
    let mut raw = [0u8; 18];
    raw[3] = 0xFA;
    // Fill all 6 modes with 0x01, 0x01 (unused)
    for i in 0..6 {
        raw[5 + i * 2] = 0x01;
        raw[6 + i * 2] = 0x01;
    }

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::StdTimings(std2) = monitor.descriptor() {
        assert_eq!(std2.iter().count(), 0);
    } else {
        panic!("expected StdTimings2, got {:?}", monitor.descriptor());
    }
}
