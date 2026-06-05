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
        Some(Descriptor::Timing(timing)) => assert_eq!(timing.pixel_clock_khz(), 74_250),
        _ => panic!("slot 1 should parse as timing"),
    }
}

#[test]
fn validate_rejects_missing_preferred_timing() {
    let raw = [0_u8; 128];
    let validation = Descriptors::new(&raw, false).validate();

    assert_eq!(
        validation.errors & (1 << (FailureKind::DescriptorFirstNotDetailedTiming as u8)),
        1 << (FailureKind::DescriptorFirstNotDetailedTiming as u8),
        "{}",
        FailureKind::DescriptorFirstNotDetailedTiming.message()
    );
}
