use edid_info::edid::chroma::Chroma;

const ACER: &[u8] = include_bytes!("data/acer_ek221q_h.edid");
const ASUS: &[u8] = include_bytes!("data/asus_rog_pg27u.edid");

#[test]
fn parse_chroma_acer_ek221q_h() {
    let raw: &[u8; 10] = ACER[25..35].try_into().expect("chroma bytes");
    let out = Chroma::parse(raw);

    assert_eq!(out.red().x(), 662);
    assert_eq!(out.red().y(), 355);
    assert_eq!(out.green().x(), 330);
    assert_eq!(out.green().y(), 645);
    assert_eq!(out.blue().x(), 159);
    assert_eq!(out.blue().y(), 62);
    assert_eq!(out.white().x(), 321);
    assert_eq!(out.white().y(), 337);
}

#[test]
fn parse_chroma_asus_rog_pg27u() {
    let raw: &[u8; 10] = ASUS[25..35].try_into().expect("chroma bytes");
    let out = Chroma::parse(raw);

    assert_eq!(out.red().x(), 690);
    assert_eq!(out.red().y(), 322);
    assert_eq!(out.green().x(), 198);
    assert_eq!(out.green().y(), 717);
    assert_eq!(out.blue().x(), 154);
    assert_eq!(out.blue().y(), 49);
    assert_eq!(out.white().x(), 320);
    assert_eq!(out.white().y(), 337);
}
