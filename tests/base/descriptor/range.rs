use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};
use edid_info::base::descriptor::range::{RateRange, Scaling, VideoTimingSupport};
use edid_info::common::AspectRatio;

const ACER: &[u8] = include_bytes!("../../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../../data/ASUS_ROG_PG27U.edid");
const PHL: &[u8] = include_bytes!("../../data/PHL_22PFL3606.edid");
const SDC: &[u8] = include_bytes!("../../data/SDC_123YL01.edid");
const ROL: &[u8] = include_bytes!("../../data/ROL_ROLSEN_C707N.edid");

#[test]
fn parse_range_limit_descriptor_acer_ek221q_h() {
    let range_raw: [u8; 18] = std::array::from_fn(|i| ACER[108 + i]);
    let range = Monitor::new(&range_raw, false);
    let desc = range.descriptor();
    assert!(matches!(desc, DisplayDescriptor::RangeLimits(_)));

    if let DisplayDescriptor::RangeLimits(range) = desc {
        assert_eq!(range.vertical_hz(), RateRange { min: 48, max: 100 });
        assert_eq!(range.horizontal_khz(), RateRange { min: 24, max: 120 });
        assert_eq!(range.max_pixel_clock_mhz(), 250);
        assert_eq!(range.timing(), VideoTimingSupport::DefaultGtf);
    }
}

#[test]
fn parse_range_limit_descriptor_asus_rog_pg27u() {
    let range_raw: [u8; 18] = std::array::from_fn(|i| ASUS[90 + i]);
    let range = Monitor::new(&range_raw, false);
    let DisplayDescriptor::RangeLimits(range) = range.descriptor() else {
        panic!()
    };
    assert_eq!(range.vertical_hz(), RateRange { min: 24, max: 144 });
    assert_eq!(range.horizontal_khz(), RateRange { min: 52, max: 322 });
    assert_eq!(range.max_pixel_clock_mhz(), 1270);
    assert_eq!(range.timing(), VideoTimingSupport::RangeLimitsOnly);
}

#[test]
fn parse_range_secondary_gtf_phl_22pfl3606() {
    let range_raw: [u8; 18] = std::array::from_fn(|i| PHL[90 + i]);
    let range = Monitor::new(&range_raw, false);

    let DisplayDescriptor::RangeLimits(range) = range.descriptor() else {
        panic!()
    };
    assert_eq!(range.vertical_hz(), RateRange { min: 55, max: 76 });
    assert_eq!(range.horizontal_khz(), RateRange { min: 15, max: 68 });
    assert_eq!(range.max_pixel_clock_mhz(), 150);

    match range.timing() {
        VideoTimingSupport::GtfSecondaryCurve(sgtf) => {
            assert_eq!(sgtf.start_khz(), 64);
            assert!((sgtf.c() - 16.0).abs() < f32::EPSILON);
            assert_eq!(sgtf.m(), 8224);
            assert_eq!(sgtf.k(), 32);
            assert!((sgtf.j() - 16.0).abs() < f32::EPSILON);
        }
        _ => panic!("Expected GtfSecondaryCurve timing"),
    }
}

#[test]
fn parse_range_cvt_sdc_123yl01() {
    let range_raw: [u8; 18] = std::array::from_fn(|i| SDC[72 + i]);
    let range = Monitor::new(&range_raw, false);

    let DisplayDescriptor::RangeLimits(range) = range.descriptor() else {
        panic!()
    };
    assert_eq!(range.vertical_hz(), RateRange { min: 48, max: 60 });
    assert_eq!(range.horizontal_khz(), RateRange { min: 0, max: 0 });
    assert_eq!(range.max_pixel_clock_mhz(), 340);

    match range.timing() {
        VideoTimingSupport::Cvt(cvt) => {
            assert_eq!(cvt.version().major, 0);
            assert_eq!(cvt.version().minor, 10);
            assert_eq!(cvt.pixel_clock_precision_khz(), 1250);
            assert_eq!(cvt.max_horizontal_active(), 160);
            let ars: Vec<_> = cvt.aspect_ratios().collect();
            assert_eq!(ars, vec![AspectRatio::new(5, 4)]);
            assert_eq!(cvt.preferred_aspect(), Some(AspectRatio::new(4, 3)));
            assert!(cvt.blanking().reduced);
            assert!(!cvt.blanking().standard);
            assert_eq!(cvt.horizontal_scaling(), Scaling::None);
            assert_eq!(cvt.vertical_scaling(), Scaling::Stretch);
            assert_eq!(cvt.preferred_vertical_hz(), 20);
        }
        _ => panic!("Expected Cvt timing"),
    }
}

#[test]
fn parse_range_limits_descriptor_rol_rolsen_c707n() {
    let range_raw: [u8; 18] = std::array::from_fn(|i| ROL[108 + i]);
    let range = Monitor::new(&range_raw, false);

    let DisplayDescriptor::RangeLimits(vals) = range.descriptor() else {
        panic!()
    };
    assert_eq!(vals.vertical_hz(), RateRange { min: 50, max: 90 });
    assert_eq!(vals.horizontal_khz(), RateRange { min: 30, max: 88 });
    assert_eq!(vals.max_pixel_clock_mhz(), 180);
    assert_eq!(vals.timing(), VideoTimingSupport::DefaultGtf);
}
