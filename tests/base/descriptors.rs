use edid_info::base::descriptors::{DTD_OFF, Descriptors, Mode};
use edid_info::common::DESC_LEN;

#[test]
fn parse_dtd_synthetic() {
    let mut raw = [0_u8; 128];
    let off = DTD_OFF + DESC_LEN;
    raw[off] = 1;
    raw[off + 1] = 29;

    let out = Descriptors::new(&raw);
    match out.mode(1) {
        Some(Mode::Timing(timing)) => assert_eq!(timing.pixel_clock_hz(), 74_250_000),
        _ => panic!("slot 1 should parse as timing"),
    }
}
