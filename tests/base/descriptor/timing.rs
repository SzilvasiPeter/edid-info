use edid_info::base::descriptor::timing::{DetailedTiming, Stereo, Sync};
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
    assert!(!out.features().interlaced());
    // flag 0x38: 0011 1000 -> bits 6:5 = 01, bit 0 = 0 -> FieldSeqRight
    assert_eq!(out.features().stereo(), Stereo::FieldSeqRight);
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
    assert_eq!(h.size_mm(), 479);
    assert_eq!(v.size_mm(), 260);
    assert_eq!(h.front(), 88);
    assert_eq!(h.sync(), 44);
    assert_eq!(h.back(), 148);
    assert_eq!(v.front(), 4);
    assert_eq!(v.sync(), 5);
    assert_eq!(v.back(), 36);
    assert_eq!(h.border(), 0);
    assert_eq!(v.border(), 0);
    assert!(!out.features().interlaced());
    assert_eq!(out.features().stereo(), Stereo::None);
    assert_eq!(
        out.features().sync(),
        Sync::DigitalSeparate {
            v_polar: true,
            h_polar: true,
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
    assert_eq!(h.size_mm(), 598);
    assert_eq!(v.size_mm(), 336);
    assert_eq!(h.front(), 48);
    assert_eq!(h.sync(), 32);
    assert_eq!(h.back(), 80);
    assert_eq!(v.front(), 3);
    assert_eq!(v.sync(), 5);
    assert_eq!(v.back(), 54);
    assert!(!out.features().interlaced());
    assert_eq!(out.features().stereo(), Stereo::None);
    assert_eq!(
        out.features().sync(),
        Sync::DigitalSeparate {
            v_polar: false,
            h_polar: true,
        }
    );
}

#[test]
fn test_interlaced_stereo_parsing() {
    // DTD 2 from PHL_22PFL3606: flag byte 0x9e = 1001 1110
    // bit 7 = 1 → interlaced, bits 6:5 = 00 → Stereo::None
    let raw_cta: [u8; 128] = std::array::from_fn(|i| PHL[128 + i]);
    let Extension::Cta(cta) = Extension::parse(&raw_cta) else {
        panic!("expected CTA extension")
    };

    let dtd2 = cta.dtd(2).expect("dtd 2");
    assert!(dtd2.features().interlaced());
    assert_eq!(dtd2.features().stereo(), Stereo::None);

    // DTD 3: flag byte 0x98 = 1001 1000
    // bit 7 = 1 → interlaced, bits 6:5 = 00 → Stereo::None
    let dtd3 = cta.dtd(3).expect("dtd 3");
    assert!(dtd3.features().interlaced());
    assert_eq!(dtd3.features().stereo(), Stereo::None);
}
