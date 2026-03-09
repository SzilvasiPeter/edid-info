use edid_info::edid::descriptor::monitor::{DescTag, MonitorDesc};
use edid_info::edid::descriptor::range::{AspectPref, Range, Timing};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");
const PHL: &[u8] = include_bytes!("../data/PHL_22PFL3606.edid");
const SDC: &[u8] = include_bytes!("../data/SDC_123YL01.edid");

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

#[test]
fn parse_range_secondary_gtf_phl_22pfl3606() {
    let range_raw: &[u8; 18] = PHL[90..108].try_into().expect("range descriptor bytes");
    let range = MonitorDesc::parse(range_raw).expect("range descriptor parse");
    assert_eq!(range.tag(), DescTag::RangeLimits);

    let range = Range::parse(range_raw).expect("range limits parse");
    assert_eq!(range.v_min_hz(), 55);
    assert_eq!(range.v_max_hz(), 76);
    assert_eq!(range.h_min_khz(), 15);
    assert_eq!(range.h_max_khz(), 68);
    assert_eq!(range.pixel_mhz(), 150);

    match range.timing() {
        Timing::SecondaryGtf(sgtf) => {
            assert_eq!(sgtf.start_khz(), 64);
            assert_eq!(sgtf.c_x2(), 32);
            assert_eq!(sgtf.m(), 8224);
            assert_eq!(sgtf.k(), 32);
            assert_eq!(sgtf.j_x2(), 32);
        }
        _ => panic!("Expected SecondaryGtf timing"),
    }
}

#[test]
fn parse_range_cvt_sdc_123yl01() {
    let range_raw: &[u8; 18] = SDC[72..90].try_into().expect("range descriptor bytes");
    let range = MonitorDesc::parse(range_raw).expect("range descriptor parse");
    assert_eq!(range.tag(), DescTag::RangeLimits);

    let range = Range::parse(range_raw).expect("range limits parse");
    assert_eq!(range.v_min_hz(), 48);
    assert_eq!(range.v_max_hz(), 60);
    assert_eq!(range.h_min_khz(), 0);
    assert_eq!(range.h_max_khz(), 0);
    assert_eq!(range.pixel_mhz(), 340);

    match range.timing() {
        Timing::Cvt(cvt) => {
            assert_eq!(cvt.major(), 0);
            assert_eq!(cvt.minor(), 10);
            assert_eq!(cvt.add_clock_0_25_mhz(), 5);
            assert_eq!(cvt.max_active(), Some(20));
            assert!(!cvt.ar_4_3());
            assert!(!cvt.ar_16_9());
            assert!(!cvt.ar_16_10());
            assert!(cvt.ar_5_4());
            assert!(!cvt.ar_15_9());
            assert_eq!(cvt.pref(), AspectPref::A4_3);
            assert!(cvt.rb());
            assert!(!cvt.std_blank());
            assert!(!cvt.h_shrink());
            assert!(!cvt.h_stretch());
            assert!(!cvt.v_shrink());
            assert!(cvt.v_stretch());
            assert_eq!(cvt.pref_v_hz(), 20);
        }
        _ => panic!("Expected Cvt timing"),
    }
}
