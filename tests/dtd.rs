use edid_info::edid::dtd::{DTD_LEN, DTD_OFF, Dtd, Mode};

#[test]
fn parse_dtd_synthetic() {
    let mut raw = [0_u8; 128];
    let off = DTD_OFF + DTD_LEN;
    raw[off] = 1;
    raw[off + 1] = 29;

    let out = Dtd::parse_base(&raw);
    match out.mode(1) {
        Some(Mode::Timing(timing)) => assert_eq!(timing.pixel_clock_hz(), 74_250_000),
        _ => panic!("slot 1 should parse as timing"),
    }
}
