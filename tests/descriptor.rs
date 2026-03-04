use edid_info::edid::descriptor::monitor::{DescTag, MonitorDesc};
use edid_info::edid::descriptor::range::{Range, Timing};
use edid_info::edid::descriptor::timing::{DetailedTiming, Stereo, Sync};

const EDID: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_timing_descriptor_from_real_edid() {
    assert_eq!(EDID.len(), 256);
    let raw = EDID[54..72].try_into().expect("timing descriptor bytes");
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
fn parse_serial_descriptor_from_real_edid() {
    assert_eq!(EDID.len(), 256);
    let serial_raw: &[u8; 18] = EDID[72..90].try_into().expect("serial descriptor bytes");

    let serial = MonitorDesc::parse(serial_raw).expect("serial descriptor parse");
    assert_eq!(serial.tag(), DescTag::SerialNumber);
    assert_eq!(serial.serial(), Some("13480002C3W01"));
    assert_eq!(serial.name(), None);
    assert_eq!(serial.text(), None);
}

#[test]
fn parse_product_name_descriptor_from_real_edid() {
    assert_eq!(EDID.len(), 256);
    let name_raw: &[u8; 18] = EDID[90..108].try_into().expect("name descriptor bytes");
    let name = MonitorDesc::parse(name_raw).expect("name descriptor parse");
    assert_eq!(name.tag(), DescTag::MonitorName);
    assert_eq!(name.name(), Some("EK221Q H"));
    assert_eq!(name.serial(), None);
    assert_eq!(name.text(), None);
}

#[test]
fn parse_range_limit_descriptor_from_real_edid() {
    assert_eq!(EDID.len(), 256);
    let range_raw: &[u8; 18] = EDID[108..126].try_into().expect("range descriptor bytes");
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
