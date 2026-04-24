use edid_info::base::descriptor::dtd::{DetailedTiming, Stereo, Sync};
use edid_info::extensions::Extension;

const ACER: &[u8] = include_bytes!("../../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../../data/ASUS_ROG_PG27U.edid");
const PRE: &[u8] = include_bytes!("../../data/PRE_P1710.edid");
const PHL: &[u8] = include_bytes!("../../data/PHL_22PFL3606.edid");

#[test]
fn parse_timing_descriptor_pre_p1710_stereo() {
    let raw = std::array::from_fn(|i| PRE[54 + i]);
    let out = DetailedTiming::parse(&raw).expect("detailed timing parse");

    assert_eq!(out.horizontal().active(), 1280);
    assert_eq!(out.vertical().active(), 1024);
    assert!(!out.interlaced());
    assert_eq!(out.stereo(), Stereo::FieldSeqRight);
}

#[test]
fn parse_timing_descriptor_acer_ek221q_h() {
    let raw = std::array::from_fn(|i| ACER[54 + i]);
    let out = DetailedTiming::parse(&raw);
    assert!(out.is_some());

    let out = out.expect("detailed timing parse");
    let h = out.horizontal();
    let v = out.vertical();
    assert_eq!(h.active(), 1920);
    assert_eq!(v.active(), 1080);
    assert_eq!(out.pixel_clock_hz(), 148_500_000);
    assert_eq!(out.physical().width(), 479);
    assert_eq!(out.physical().height(), 260);
    assert_eq!(h.front(), 88);
    assert_eq!(h.sync(), 44);
    assert_eq!(h.back(), 148);
    assert_eq!(v.front(), 4);
    assert_eq!(v.sync(), 5);
    assert_eq!(v.back(), 36);
    assert_eq!(h.border(), 0);
    assert_eq!(v.border(), 0);
    assert!(!out.interlaced());
    assert_eq!(out.stereo(), Stereo::None);
    assert_eq!(
        out.sync(),
        Sync::DigitalSeparate {
            v_positive: true,
            h_positive: true,
        }
    );
}

#[test]
fn parse_timing_descriptor_asus_rog_pg27u() {
    let raw = std::array::from_fn(|i| ASUS[54 + i]);
    let out = DetailedTiming::parse(&raw);
    assert!(out.is_some());

    let out = out.expect("detailed timing parse");
    let h = out.horizontal();
    let v = out.vertical();
    assert_eq!(h.active(), 3840);
    assert_eq!(v.active(), 2160);
    assert_eq!(out.pixel_clock_hz(), 533_250_000);
    assert_eq!(out.physical().width(), 598);
    assert_eq!(out.physical().height(), 336);
    assert_eq!(h.front(), 48);
    assert_eq!(h.sync(), 32);
    assert_eq!(h.back(), 80);
    assert_eq!(v.front(), 3);
    assert_eq!(v.sync(), 5);
    assert_eq!(v.back(), 54);
    assert!(!out.interlaced());
    assert_eq!(out.stereo(), Stereo::None);
    assert_eq!(
        out.sync(),
        Sync::DigitalSeparate {
            v_positive: false,
            h_positive: true,
        }
    );
}

#[test]
fn test_interlaced_stereo_parsing() {
    let raw_cta: [u8; 128] = std::array::from_fn(|i| PHL[128 + i]);
    let Extension::Cta(cta) = Extension::parse(&raw_cta) else {
        panic!("expected CTA extension")
    };

    let dtd2 = cta.dtd(2).expect("dtd 2");
    assert!(dtd2.interlaced());
    assert_eq!(dtd2.stereo(), Stereo::None);

    let dtd3 = cta.dtd(3).expect("dtd 3");
    assert!(dtd3.interlaced());
    assert_eq!(dtd3.stereo(), Stereo::None);
}
