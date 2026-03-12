use edid_info::edid::{Edid, Extension};

const ACER: &[u8] = include_bytes!("data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_edid_acer_ek221q_h() {
    assert_eq!(ACER.len(), 256);
    let edid = Edid::parse(ACER).expect("should parse Acer EDID");

    // Verify base block
    assert_eq!(edid.base().header().maker(), ['A', 'C', 'R']);
    assert_eq!(edid.base().footer().extension_num(), 1);

    // Verify extensions
    let extensions = edid.extensions();
    assert_eq!(extensions.len(), 1);
    match &extensions[0] {
        Extension::Cta(cta) => {
            assert_eq!(cta.header().rev(), 3);
        }
        Extension::Unknown(_) => panic!("expected CTA extension for Acer, got Unknown"),
    }
}

#[test]
fn parse_edid_asus_rog_pg27u() {
    assert_eq!(ASUS.len(), 768);
    let edid = Edid::parse(ASUS).expect("should parse Asus EDID");

    // Verify base block
    assert_eq!(edid.base().header().maker(), ['A', 'U', 'S']);
    assert_eq!(edid.base().footer().extension_num(), 2);

    // Verify extensions (footer says 2, so we expect 2 parsed extensions even if file is longer)
    let extensions = edid.extensions();
    assert_eq!(extensions.len(), 2);

    // Block 1: CTA
    match &extensions[0] {
        Extension::Cta(cta) => {
            assert_eq!(cta.header().rev(), 3);
        }
        Extension::Unknown(_) => panic!("expected CTA extension for Asus block 1, got Unknown"),
    }
}

#[test]
fn parse_edid_invalid_length() {
    let short = &ACER[..100];
    assert!(Edid::parse(short).is_none());

    let unaligned = &ACER[..200];
    assert!(Edid::parse(unaligned).is_none());
}
