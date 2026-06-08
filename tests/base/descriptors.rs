const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");

use edid_info::base::descriptors::{DESCRIPTORS_OFF, Descriptor, Descriptors};
use edid_info::common::{DESC_LEN, FailureKind};

#[test]
fn parse_dtd_synthetic() {
    let mut raw = [0_u8; 128];
    let off = DESCRIPTORS_OFF + DESC_LEN;
    raw[off] = 1;
    raw[off + 1] = 29;

    let out = Descriptors::new(&raw, false);
    match out.iter().nth(1) {
        Some(Some(Descriptor::Timing(timing))) => assert_eq!(timing.pixel_clock_khz(), 74_250),
        _ => panic!("slot 1 should parse as timing"),
    }
}

#[test]
fn validate_rejects_missing_preferred_timing() {
    let raw = [0_u8; 128];
    let validation = Descriptors::new(&raw, false).validate(false);

    assert_eq!(
        validation.errors & (1 << (FailureKind::FirstDescriptorNotDetailedTiming as u8)),
        1 << (FailureKind::FirstDescriptorNotDetailedTiming as u8),
        "{}",
        FailureKind::FirstDescriptorNotDetailedTiming.message()
    );
}

#[test]
fn validate_rejects_timing_after_display_descriptor() {
    let mut raw = [0_u8; 128];
    raw.copy_from_slice(&ACER[..128]);

    let mut dtd = [0_u8; DESC_LEN];
    dtd.copy_from_slice(&raw[DESCRIPTORS_OFF..DESCRIPTORS_OFF + DESC_LEN]);

    let later_off = DESCRIPTORS_OFF + 2 * DESC_LEN;
    raw[later_off..later_off + DESC_LEN].copy_from_slice(&dtd);

    let validation = Descriptors::new(&raw, false).validate(false);

    assert_eq!(
        validation.errors & (1 << (FailureKind::InvalidDescriptorOrder as u8)),
        1 << (FailureKind::InvalidDescriptorOrder as u8),
        "{}",
        FailureKind::InvalidDescriptorOrder.message()
    );
}
