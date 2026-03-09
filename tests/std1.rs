use edid_info::edid::std1::{Aspect, Std1};

const ACER: &[u8] = include_bytes!("data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_standard_acer_ek221q_h() {
    let raw: &[u8; 16] = ACER[38..54].try_into().expect("standard bytes");
    let mode = |i| {
        Std1::parse(raw)
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

#[test]
fn parse_standard_asus_rog_pg27u() {
    let raw: &[u8; 16] = ASUS[38..54].try_into().expect("standard bytes");
    let mode = |i| {
        Std1::parse(raw)
            .mode(i)
            .map(|t| (t.width(), t.height(), t.aspect(), t.vfreq()))
    };

    assert_eq!(mode(0), None);
    assert_eq!(mode(1), None);
    assert_eq!(mode(2), None);
    assert_eq!(mode(3), None);
    assert_eq!(mode(4), None);
    assert_eq!(mode(5), None);
    assert_eq!(mode(6), None);
    assert_eq!(mode(7), None);
}
