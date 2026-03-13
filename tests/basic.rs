use edid_info::edid::basic::{Basic, BitDepth, DigitalType, DisplayType, InputKind, Interface};

const ACER: &[u8] = include_bytes!("data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_basic_acer_ek221q_h() {
    assert_eq!(ACER.len(), 256);
    let raw: [u8; 5] = std::array::from_fn(|i| ACER[20 + i]);
    let out = Basic::parse(&raw);

    assert_eq!(
        out.input().kind(),
        InputKind::Digital {
            depth: BitDepth::Undef,
            iface: Interface::Undef,
        }
    );
    assert_eq!(out.width_cm(), 48);
    assert_eq!(out.height_cm(), 26);
    assert_eq!(out.gamma_raw(), 120);
    assert!(out.features().standby());
    assert!(out.features().suspend());
    assert!(out.features().active_off());
    assert_eq!(
        out.features().display(),
        DisplayType::Digital(DigitalType::Rgb444Y444)
    );
    assert!(!out.features().srgb());
    assert!(out.features().pref_timing_mode());
    assert!(!out.features().continuous());
}

#[test]
fn parse_basic_asus_rog_pg27u() {
    let raw: [u8; 5] = std::array::from_fn(|i| ASUS[20 + i]);
    let out = Basic::parse(&raw);

    assert_eq!(
        out.input().kind(),
        InputKind::Digital {
            depth: BitDepth::B10,
            iface: Interface::DisplayPort,
        }
    );
    assert_eq!(out.width_cm(), 60);
    assert_eq!(out.height_cm(), 34);
    assert_eq!(out.gamma_raw(), 120);
    assert!(!out.features().standby());
    assert!(!out.features().suspend());
    assert!(out.features().active_off());
    assert_eq!(
        out.features().display(),
        DisplayType::Digital(DigitalType::Rgb444Y444Y422)
    );
    assert!(!out.features().srgb());
    assert!(out.features().pref_timing_mode());
    assert!(!out.features().continuous());
}
