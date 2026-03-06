use edid_info::edid::basic::{Basic, BitDepth, DigitalType, DisplayType, InputKind, Interface};

const EDID: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_basic_acer_ek221q_h() {
    assert_eq!(EDID.len(), 256);
    let raw: &[u8; 5] = EDID[20..25].try_into().expect("basic bytes");
    let out = Basic::parse(raw);

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
    assert!(out.feat().stand());
    assert!(out.feat().susp());
    assert!(out.feat().off());
    assert_eq!(
        out.feat().display(),
        DisplayType::Digital(DigitalType::Rgb444Y444)
    );
    assert!(!out.feat().srgb());
    assert!(out.feat().pref());
    assert!(!out.feat().cont());
}
