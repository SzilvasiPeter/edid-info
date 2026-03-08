use edid_info::edid::descriptor::monitor::{DescTag, MonitorDesc};
use edid_info::edid::descriptor::range::{Range, Timing};

const ACER: &[u8] = include_bytes!("../data/acer_ek221q_h.edid");
const ASUS: &[u8] = include_bytes!("../data/asus_rog_pg27u.edid");

#[test]
fn parse_range_limit_descriptor_acer_ek221q_h() {
    let range_raw: &[u8; 18] = ACER[108..126].try_into().expect("range descriptor bytes");
    let range = MonitorDesc::parse(range_raw).expect("range descriptor parse");
    assert_eq!(range.tag(), DescTag::RangeLimits);
    assert_eq!(range.serial(), None);
    assert_eq!(range.name(), None);
    assert_eq!(range.text(), None);

    let range = Range::parse(range_raw).expect("range limits parse");
    assert_eq!(range.v_min_hz(), 48);
    assert_eq!(range.v_max_hz(), 100);
    assert_eq!(range.h_min_khz(), 24);
    assert_eq!(range.h_max_khz(), 120);
    assert_eq!(range.pixel_mhz(), 250);
    assert_eq!(range.timing(), Timing::DefaultGtf);
}

#[test]
fn parse_range_limit_descriptor_asus_rog_pg27u() {
    let range_raw: &[u8; 18] = ASUS[90..108].try_into().expect("range descriptor bytes");
    let range = MonitorDesc::parse(range_raw).expect("range descriptor parse");
    assert_eq!(range.tag(), DescTag::RangeLimits);
    assert_eq!(range.serial(), None);
    assert_eq!(range.name(), None);
    assert_eq!(range.text(), None);

    let range = Range::parse(range_raw).expect("range limits parse");
    assert_eq!(range.v_min_hz(), 24);
    assert_eq!(range.v_max_hz(), 144);
    assert_eq!(range.h_min_khz(), 52);
    assert_eq!(range.h_max_khz(), 322);
    assert_eq!(range.pixel_mhz(), 1270);
    assert_eq!(range.timing(), Timing::NoTiming);
}
