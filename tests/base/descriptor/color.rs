use edid_info::base::{
    chroma::Coord,
    descriptor::monitor::{DisplayDescriptor, Monitor},
};

// TODO: use real world example instead of synthetic
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

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::ColorPoint(wp) = monitor.descriptor() {
        let first = wp.first().expect("first point");
        assert_eq!(first.coord(), Coord { x: 0x200, y: 0x00 });
        assert!((first.gamma().unwrap() - 1.64).abs() < 0.01);

        let second = wp.second().expect("second point");
        assert_eq!(second.coord(), Coord { x: 0x101, y: 1 });
        assert!((second.gamma().unwrap() - 1.80).abs() < 0.01);
    } else {
        panic!("expected WhitePoint, got {:?}", monitor.descriptor());
    }
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

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::ColorPoint(wp) = monitor.descriptor() {
        assert!(wp.first().is_some());
        assert!(wp.second().is_none());
    } else {
        panic!("expected WhitePoint, got {:?}", monitor.descriptor());
    }
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

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::ColorPoint(wp) = monitor.descriptor() {
        let first = wp.first().expect("first point");
        assert!((first.gamma().unwrap() - 2.0).abs() < 0.01);
    } else {
        panic!("expected WhitePoint, got {:?}", monitor.descriptor());
    }
}
