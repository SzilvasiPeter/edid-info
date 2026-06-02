use edid_info::base::standard::StandardTimings;
use edid_info::common::{AspectRatio, FailureKind};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

fn base(raw: &[u8]) -> [u8; 128] {
    core::array::from_fn(|i| raw[i])
}

#[test]
fn parse_standard_acer_ek221q_h() {
    let raw = base(ACER);
    let timings = |i| {
        StandardTimings::new(&raw, false).iter().nth(i).map(|t| {
            (
                t.horizontal_active,
                t.vertical_active,
                t.aspect_ratio,
                t.refresh_rate,
            )
        })
    };

    assert_eq!(timings(0), Some((1152, 864, AspectRatio::new(4, 3), 75)));
    assert_eq!(timings(1), Some((1280, 960, AspectRatio::new(4, 3), 60)));
    assert_eq!(timings(2), Some((1280, 1024, AspectRatio::new(5, 4), 60)));
    assert_eq!(timings(3), Some((1280, 720, AspectRatio::new(16, 9), 60)));
    assert_eq!(timings(4), Some((1680, 1050, AspectRatio::new(16, 10), 60)));
    assert_eq!(timings(5), Some((1280, 800, AspectRatio::new(16, 10), 60)));
    assert_eq!(timings(6), Some((1440, 900, AspectRatio::new(16, 10), 60)));
    assert_eq!(timings(7), Some((1920, 1080, AspectRatio::new(16, 9), 75)));
    assert_eq!(timings(8), None);
}

#[test]
fn parse_standard_asus_rog_pg27u() {
    let raw = base(ASUS);
    let timings = |i| {
        StandardTimings::new(&raw, false).iter().nth(i).map(|t| {
            (
                t.horizontal_active,
                t.vertical_active,
                t.aspect_ratio,
                t.refresh_rate,
            )
        })
    };

    assert_eq!(timings(0), None);
    assert_eq!(timings(1), None);
    assert_eq!(timings(2), None);
    assert_eq!(timings(3), None);
    assert_eq!(timings(4), None);
    assert_eq!(timings(5), None);
    assert_eq!(timings(6), None);
    assert_eq!(timings(7), None);
}

#[test]
fn validate_standard_rejects_zero_empty_slot() {
    let mut raw = [0x00; 128];
    raw[38..54].fill(0x01);
    raw[38] = 0x00;
    raw[39] = 0x00;

    let validation = StandardTimings::new(&raw, false).validate();

    assert!(!validation.is_valid());
    assert_eq!(
        validation.errors,
        1 << (FailureKind::StdTimingEmptyInvalid as u8),
    );
    assert_eq!(validation.warnings, 0);
    assert_eq!(
        FailureKind::StdTimingEmptyInvalid.message(),
        "Use 0x0101 as the invalid Standard Timings code"
    );
}

#[test]
fn validate_standard_rejects_too_small_horizontal() {
    let mut raw = [0x00; 128];
    raw[38..54].fill(0x01);
    raw[38] = 0x00;
    raw[39] = 0x40;

    let validation = StandardTimings::new(&raw, false).validate();

    assert!(!validation.is_valid());
    assert_eq!(
        validation.errors,
        1 << (FailureKind::StdTimingHorizontalLimit as u8),
    );
    assert_eq!(validation.warnings, 0);
    assert_eq!(
        FailureKind::StdTimingHorizontalLimit.message(),
        "Standard timing horizontal pixels outside 256-2288"
    );
}
