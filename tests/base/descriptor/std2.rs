use edid_info::base::descriptor::monitor::Monitor;

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

    let desc = Monitor::parse(&raw).expect("monitor descriptor parse");
    let std2 = desc.std2().expect("std2 parse");

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

    let desc = Monitor::parse(&raw).expect("monitor descriptor parse");
    let std2 = desc.std2().expect("std2 parse");

    assert!(std2.mode(0).is_none());
    assert!(std2.mode(1).is_some());
    assert!(std2.mode(2).is_some());
    assert!(std2.mode(3).is_some());
    assert!(std2.mode(4).is_some());
    assert!(std2.mode(5).is_some());
    assert_eq!(std2.pad(), 0);
}
