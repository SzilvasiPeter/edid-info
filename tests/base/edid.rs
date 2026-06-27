use edid_info::edid::Edid;
use edid_info::extensions::Extension;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_edid_acer_ek221q_h() {
    assert_eq!(ACER.len(), 256);
    let edid = Edid::parse(ACER).expect("should parse Acer EDID");

    // Verify base block
    assert_eq!(edid.base().header().manufacturer(), ['A', 'C', 'R']);
    assert_eq!(edid.base().footer().extension_count(), 1);

    // Verify extensions
    let first = edid.extensions().next().expect("extension 0");
    assert!(
        matches!(first, Extension::Cta(_)),
        "expected CTA extension for Acer"
    );
    if let Extension::Cta(cta) = first {
        assert_eq!(cta.revision(), 3);
    }
}

#[test]
fn parse_edid_asus_rog_pg27u() {
    assert_eq!(ASUS.len(), 768);
    let edid = Edid::parse(ASUS).expect("should parse Asus EDID");

    // Verify base block
    assert_eq!(edid.base().header().manufacturer(), ['A', 'U', 'S']);
    assert_eq!(edid.base().footer().extension_count(), 2);

    // Verify extensions (footer says 2, so we expect 2 parsed extensions even if file is longer)
    let first = edid.extensions().next().expect("extension 0");
    assert!(
        matches!(first, Extension::Cta(_)),
        "expected CTA extension for Asus block 1"
    );
    if let Extension::Cta(cta) = first {
        assert_eq!(cta.revision(), 3);
    }
}
