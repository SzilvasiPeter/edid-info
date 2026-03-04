use edid_info::edid::standard::{Aspect, Standard};

const EDID: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_standard_happy_path_from_real_edid() {
    assert_eq!(EDID.len(), 256);
    let raw: &[u8; 16] = EDID[38..54].try_into().expect("standard bytes");
    let mode = |i| {
        Standard::parse(raw)
            .mode(i)
            .map(|t| (t.width(), t.height(), t.aspect(), t.vfreq()))
    };

    assert_eq!(mode(0), Some((1152, 864, Aspect::A4_3, 75)));
    assert_eq!(mode(1), Some((1280, 960, Aspect::A4_3, 60)));
    assert_eq!(mode(2), Some((1280, 1024, Aspect::A5_4, 60)));
    assert_eq!(mode(3), Some((1280, 720, Aspect::A16_9, 60)));
    assert_eq!(mode(4), Some((1680, 1050, Aspect::A16_10, 60)));
    assert_eq!(mode(5), Some((1280, 800, Aspect::A16_10, 60)));
    assert_eq!(mode(6), Some((1440, 900, Aspect::A16_10, 60)));
    assert_eq!(mode(7), Some((1920, 1080, Aspect::A16_9, 75)));
    assert_eq!(mode(8), None);
}
