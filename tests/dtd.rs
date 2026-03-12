use edid_info::edid::DESC_LEN;
use edid_info::edid::dtd::{DTD_OFF, Descriptors, Mode};

#[test]
fn parse_dtd_synthetic() {
    let mut raw = [0_u8; 128];
    let off = DTD_OFF + DESC_LEN;
    raw[off] = 1;
    raw[off + 1] = 29;

    let out = Descriptors::parse_base(&raw);
    match out.mode(1) {
        Some(Mode::Timing(timing)) => assert_eq!(timing.pixel_clock_hz(), 74_250_000),
        _ => panic!("slot 1 should parse as timing"),
    }
}
