use edid_info::base::{dmt::find_std, standard::StdTimings};
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
        StdTimings::new(&raw).iter().nth(i).map(|t| {
            (
                t.horizontal_active(),
                t.vertical_active(),
                t.aspect_ratio(),
                t.refresh_rate(),
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
        StdTimings::new(&raw).iter().nth(i).map(|t| {
            (
                t.horizontal_active(),
                t.vertical_active(),
                t.aspect_ratio(),
                t.refresh_rate(),
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
fn parse_standard_timing_code() {
    let raw = base(ACER);

    let code = StdTimings::new(&raw)
        .iter()
        .next()
        .map(|t| t.standard_timing_code());

    assert_eq!(code, Some(0x714f));
}

#[test]
fn parse_standard_timing_codes_find_dmt_entries() {
    let raw = base(ACER);
    let expected_dmt_ids = [
        Some(0x15),
        Some(0x20),
        Some(0x23),
        Some(0x55),
        Some(0x3A),
        Some(0x1C),
        Some(0x2F),
        None,
    ];

    let mut count = 0;
    for (timing, expected_id) in StdTimings::new(&raw).iter().zip(expected_dmt_ids) {
        count += 1;
        let dmt = find_std(timing.standard_timing_code());
        assert_eq!(dmt.map(|dmt| dmt.id), expected_id);

        if let Some(dmt) = dmt {
            assert_eq!(dmt.std_code, Some(timing.standard_timing_code()));
            assert_eq!(dmt.horizontal.active(), timing.horizontal_active());
            assert_eq!(dmt.vertical.active(), timing.vertical_active());
        }
    }
    assert_eq!(count, expected_dmt_ids.len());
}

#[test]
fn validate_standard_rejects_zero_empty_slot() {
    let mut raw = [0x00; 128];
    raw[38..54].fill(0x01);
    raw[38] = 0x00;
    raw[39] = 0x00;

    let validation = StdTimings::new(&raw).validate();

    assert!(!validation.is_valid());
    assert_eq!(
        validation.errors,
        1 << (FailureKind::InvalidEmptyStdTiming as u8),
    );
    assert_eq!(validation.warnings, 0);
    assert_eq!(
        FailureKind::InvalidEmptyStdTiming.message(),
        "Use 0x01 0x01 byte code for empty Standard Timings"
    );
}

#[test]
fn validate_standard_rejects_too_small_horizontal() {
    let mut raw = [0x00; 128];
    raw[38..54].fill(0x01);
    raw[38] = 0x00;
    raw[39] = 0x40;

    let validation = StdTimings::new(&raw).validate();

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
