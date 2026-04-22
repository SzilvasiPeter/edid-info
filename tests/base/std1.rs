use edid_info::base::std1::Std1;
use edid_info::common::AspectRatio;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

fn base(raw: &[u8]) -> [u8; 128] {
    core::array::from_fn(|i| raw[i])
}

#[test]
fn parse_standard_acer_ek221q_h() {
    let raw = base(ACER);
    let mode = |i| {
        Std1::new(&raw)
            .mode(i)
            .map(|t| (t.width(), t.height(), t.aspect(), t.vfreq()))
    };

    assert_eq!(mode(0), Some((1152, 864, AspectRatio::new(4, 3), 75)));
    assert_eq!(mode(1), Some((1280, 960, AspectRatio::new(4, 3), 60)));
    assert_eq!(mode(2), Some((1280, 1024, AspectRatio::new(5, 4), 60)));
    assert_eq!(mode(3), Some((1280, 720, AspectRatio::new(16, 9), 60)));
    assert_eq!(mode(4), Some((1680, 1050, AspectRatio::new(16, 10), 60)));
    assert_eq!(mode(5), Some((1280, 800, AspectRatio::new(16, 10), 60)));
    assert_eq!(mode(6), Some((1440, 900, AspectRatio::new(16, 10), 60)));
    assert_eq!(mode(7), Some((1920, 1080, AspectRatio::new(16, 9), 75)));
    assert_eq!(mode(8), None);
}

#[test]
fn parse_standard_asus_rog_pg27u() {
    let raw = base(ASUS);
    let mode = |i| {
        Std1::new(&raw)
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
