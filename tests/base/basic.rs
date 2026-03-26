use edid_info::base::basic::{
    AnalogType, Basic, BitDepth, DigitalType, DisplayType, InputKind, Interface, Level,
};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");
const PHL: &[u8] = include_bytes!("../data/PHL_221V8.edid");
const ROL: &[u8] = include_bytes!("../data/ROL_ROLSEN_C707N.edid");
const TSB: &[u8] = include_bytes!("../data/TSB_TV.edid");
const VIT: &[u8] = include_bytes!("../data/VIT_VT988.edid");
const WG: &[u8] = include_bytes!("../data/WG@_UNKNOWN.edid");

fn base(raw: &[u8]) -> [u8; 128] {
    core::array::from_fn(|i| raw[i])
}

#[test]
fn parse_basic_acer_ek221q_h() {
    assert_eq!(ACER.len(), 256);
    let raw = base(ACER);
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
    assert_eq!(out.gamma(), Some(220));
    assert!(out.features().standby());
    assert!(out.features().suspend());
    assert!(out.features().active_off());
    assert_eq!(
        out.features().display(),
        DisplayType::Digital(DigitalType::Rgb444Y444)
    );
    assert!(!out.features().standard_rgb());
    assert!(out.features().timing_mode());
    assert!(!out.features().timing_continuous());

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(validation.warnings, 0);
}

#[test]
fn parse_basic_asus_rog_pg27u() {
    let raw = base(ASUS);
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
    assert_eq!(out.gamma(), Some(220));
    assert!(!out.features().standby());
    assert!(!out.features().suspend());
    assert!(out.features().active_off());
    assert_eq!(
        out.features().display(),
        DisplayType::Digital(DigitalType::Rgb444Y444Y422)
    );
    assert!(!out.features().standard_rgb());
    assert!(out.features().timing_mode());
    assert!(!out.features().timing_continuous());

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(validation.warnings, 0);
}

#[test]
fn parse_basic_phl_221v8() {
    let raw = base(PHL);
    let out = Basic::parse(&raw);

    assert_eq!(
        out.input().kind(),
        InputKind::Analog {
            level: Level::V700_000,
            blank_to_black: false,
            separate_sync: true,
            composite_sync: false,
            sync_on_green: false,
            serrated_sync: false,
        }
    );
    assert_eq!(out.width_cm(), 48);
    assert_eq!(out.height_cm(), 27);
    assert_eq!(out.gamma(), Some(220));
    assert!(!out.features().standby());
    assert!(!out.features().suspend());
    assert!(out.features().active_off());
    assert_eq!(
        out.features().display(),
        DisplayType::Analog(AnalogType::Rgb)
    );
    assert!(!out.features().standard_rgb());
    assert!(out.features().timing_mode());
    assert!(!out.features().timing_continuous());

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(validation.warnings, 0);
}

#[test]
fn parse_basic_rol_rolsen_c707n() {
    let raw = base(ROL);
    let out = Basic::parse(&raw);

    assert_eq!(
        out.input().kind(),
        InputKind::Analog {
            level: Level::V700_300,
            blank_to_black: false,
            separate_sync: true,
            composite_sync: false,
            sync_on_green: false,
            serrated_sync: false,
        }
    );
    assert_eq!(out.width_cm(), 30);
    assert_eq!(out.height_cm(), 22);
    assert_eq!(out.gamma(), Some(280));
    assert!(out.features().standby());
    assert!(out.features().suspend());
    assert!(!out.features().active_off());
    assert_eq!(
        out.features().display(),
        DisplayType::Analog(AnalogType::Rgb)
    );
    assert!(!out.features().standard_rgb());
    assert!(!out.features().timing_mode());
    assert!(!out.features().timing_continuous());

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(validation.warnings, 0);
}

#[test]
fn parse_basic_tsb_tv() {
    let raw = base(TSB);
    let out = Basic::parse(&raw);

    assert_eq!(
        out.input().kind(),
        InputKind::Analog {
            level: Level::V700_300,
            blank_to_black: false,
            separate_sync: true,
            composite_sync: false,
            sync_on_green: false,
            serrated_sync: false,
        }
    );
    assert_eq!(out.width_cm(), 89);
    assert_eq!(out.height_cm(), 50);
    assert_eq!(out.gamma(), Some(220));
    assert!(!out.features().standby());
    assert!(!out.features().suspend());
    assert!(!out.features().active_off());
    assert_eq!(
        out.features().display(),
        DisplayType::Analog(AnalogType::Rgb)
    );
    assert!(!out.features().standard_rgb());
    assert!(out.features().timing_mode());
    assert!(!out.features().timing_continuous());

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(validation.warnings, 0);
}

#[test]
fn parse_basic_vit_vt988() {
    let raw = base(VIT);
    let out = Basic::parse(&raw);

    assert_eq!(
        out.input().kind(),
        InputKind::Analog {
            level: Level::V700_300,
            blank_to_black: false,
            separate_sync: false,
            composite_sync: false,
            sync_on_green: false,
            serrated_sync: false,
        }
    );
    assert_eq!(out.width_cm(), 34);
    assert_eq!(out.height_cm(), 27);
    assert_eq!(out.gamma(), None);
    assert!(out.features().standby());
    assert!(out.features().suspend());
    assert!(out.features().active_off());
    assert_eq!(
        out.features().display(),
        DisplayType::Analog(AnalogType::Rgb)
    );
    assert!(!out.features().standard_rgb());
    assert!(out.features().timing_mode());
    assert!(!out.features().timing_continuous());

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(validation.warnings, 0);
}

#[test]
fn parse_basic_wg_unknown() {
    let raw = base(WG);
    let out = Basic::parse(&raw);

    assert_eq!(
        out.input().kind(),
        InputKind::Analog {
            level: Level::V700_300,
            blank_to_black: false,
            separate_sync: true,
            composite_sync: true,
            sync_on_green: false,
            serrated_sync: false,
        }
    );
    assert_eq!(out.width_cm(), 38);
    assert_eq!(out.height_cm(), 30);
    assert_eq!(out.gamma(), Some(220));
    assert!(!out.features().standby());
    assert!(!out.features().suspend());
    assert!(!out.features().active_off());
    assert_eq!(
        out.features().display(),
        DisplayType::Analog(AnalogType::Rgb)
    );
    assert!(!out.features().standard_rgb());
    assert!(!out.features().timing_mode());
    assert!(!out.features().timing_continuous());

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(validation.warnings, 0);
}
