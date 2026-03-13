use edid_info::edid::descriptor::timing::{DetailedTiming, Stereo, Sync};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_timing_descriptor_acer_ek221q_h() {
    let raw = std::array::from_fn(|i| ACER[54 + i]);
    let out = DetailedTiming::parse(&raw);
    assert!(out.is_some());

    let out = out.expect("detailed timing parse");
    assert_eq!(out.h_active(), 1920);
    assert_eq!(out.v_active(), 1080);
    assert!((out.v_hz() - 60.0).abs() < 1e-9);
    assert!((out.h_khz() - 67.5).abs() < 1e-9);
    assert_eq!(out.pixel_clock_hz(), 148_500_000);
    assert_eq!(out.h_size_mm(), 479);
    assert_eq!(out.v_size_mm(), 260);
    assert_eq!(out.h_front(), 88);
    assert_eq!(out.h_sync(), 44);
    assert_eq!(out.h_back(), 148);
    assert_eq!(out.v_front(), 4);
    assert_eq!(out.v_sync(), 5);
    assert_eq!(out.v_back(), 36);
    assert_eq!(out.h_border(), 0);
    assert_eq!(out.v_border(), 0);
    assert!(!out.feat().interlaced());
    assert_eq!(out.feat().stereo(), Stereo::None);
    assert_eq!(
        out.feat().sync(),
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
    assert_eq!(out.h_active(), 3840);
    assert_eq!(out.v_active(), 2160);
    assert!((out.v_hz() - 60.0).abs() < 0.01);
    assert_eq!(out.pixel_clock_hz(), 533_250_000);
    assert_eq!(out.h_size_mm(), 598);
    assert_eq!(out.v_size_mm(), 336);
    assert_eq!(out.h_front(), 48);
    assert_eq!(out.h_sync(), 32);
    assert_eq!(out.h_back(), 80);
    assert_eq!(out.v_front(), 3);
    assert_eq!(out.v_sync(), 5);
    assert_eq!(out.v_back(), 54);
    assert!(!out.feat().interlaced());
    assert_eq!(out.feat().stereo(), Stereo::None);
    assert_eq!(
        out.feat().sync(),
        Sync::DigitalSeparate {
            v_polar: false,
            h_polar: true,
        }
    );
}

#[test]
fn test_timing_frame_rate_interlaced() {
    let mut raw = [0u8; 18];
    raw[0] = 0x01; // clock
    raw[2] = 0x10; // h active
    raw[3] = 0x10; // h blank
    raw[5] = 0x10; // v active
    raw[6] = 0x10; // v blank
    raw[17] = 0x80; // interlaced

    let dt = DetailedTiming::parse(&raw).expect("parse");
    assert!(dt.feat().interlaced());
    let v_hz = dt.v_hz();
    let frame_hz = dt.frame_rate_hz();
    assert!((frame_hz - v_hz / 2.0).abs() < 1e-9);
}
