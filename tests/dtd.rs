use edid_info::edid::descriptor::DESC_LEN;
use edid_info::edid::dtd::{DTD_NUM, DTD_OFF, Descriptors, Mode};

#[test]
fn parse_dtd_synthetic() {
    let mut raw = [0_u8; 128];
    let off = DTD_OFF + DESC_LEN;
    raw[off] = 1;
    raw[off + 1] = 29;

    let dtd_bytes: [u8; DTD_NUM * DESC_LEN] = std::array::from_fn(|i| raw[DTD_OFF + i]);
    let out = Descriptors::parse(&dtd_bytes);
    match out.mode(1) {
        Some(Mode::Timing(timing)) => assert_eq!(timing.pixel_clock_hz(), 74_250_000),
        _ => panic!("slot 1 should parse as timing"),
    }
}
