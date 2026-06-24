use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};
use edid_info::common::WarningKind;

const ACER: &[u8] = include_bytes!("../../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_std3_not_present_acer_ek221q_h() {
    let raw: [u8; 18] = std::array::from_fn(|i| ACER[90 + i]);
    let monitor = Monitor::new(&raw, false);
    assert!(!matches!(
        monitor.descriptor(),
        DisplayDescriptor::EstTimings(_)
    ));
}

#[test]
fn parse_std3_not_present_asus_rog_pg27u() {
    let raw: [u8; 18] = std::array::from_fn(|i| ASUS[90 + i]);
    let monitor = Monitor::new(&raw, false);
    assert!(!matches!(
        monitor.descriptor(),
        DisplayDescriptor::EstTimings(_)
    ));
}

#[test]
fn parse_std3_synthetic() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF7;
    raw[5] = 0x0A;
    // set some bits: 640x350@85 (bit 7 of byte 6), 1024x768@85 (bit 1 of byte 6)
    raw[6] = 0b1000_0010;
    // set a bit in byte 7: 1280x768@60 RB (bit 7)
    raw[7] = 0b1000_0000;

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::EstTimings(est) = monitor.descriptor() {
        let timings: Vec<_> = est.iter().collect();
        assert_eq!(timings.len(), 3, "expected 3 supported timings");
        assert_eq!(timings[0].id, 0x01);
        assert_eq!(timings[1].id, 0x13);
        assert_eq!(timings[2].id, 0x16);
        assert!(est.validate().is_valid());
    } else {
        panic!("expected EstTimings, got {:?}", monitor.descriptor());
    }
}

#[test]
fn validate_std3_rejects_bad_version() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF7;
    raw[5] = 0xFF;

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::EstTimings(est) = monitor.descriptor() {
        let v = est.validate();
        assert_has_warning(v, WarningKind::EstTimingsVersionReserved);
    } else {
        panic!("expected EstTimings, got {:?}", monitor.descriptor());
    }
}

#[test]
fn validate_std3_rejects_reserved_bits() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF7;
    raw[5] = 0x0A;
    raw[11] = 0x01; // lower nibble of byte 11 is reserved

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::EstTimings(est) = monitor.descriptor() {
        let v = est.validate();
        assert_has_warning(v, WarningKind::EstTimingsReservedBits);
    } else {
        panic!("expected EstTimings, got {:?}", monitor.descriptor());
    }
}

#[test]
fn validate_std3_rejects_reserved_bytes() {
    let mut raw = [0u8; 18];
    raw[3] = 0xF7;
    raw[5] = 0x0A;
    raw[12] = 0x01; // byte 12 is reserved

    let monitor = Monitor::new(&raw, false);
    if let DisplayDescriptor::EstTimings(est) = monitor.descriptor() {
        let v = est.validate();
        assert_has_warning(v, WarningKind::EstTimingsReservedBits);
    } else {
        panic!("expected EstTimings, got {:?}", monitor.descriptor());
    }
}

fn assert_has_warning(v: edid_info::common::Validation, kind: WarningKind) {
    assert!(
        v.warnings & (1 << kind as u8) != 0,
        "expected warning {kind:?}",
    );
}
