use edid_info::edid::chroma::Chroma;

const EDID: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_chroma_happy_path_from_real_edid() {
    assert_eq!(EDID.len(), 256);
    let raw: &[u8; 10] = EDID[25..35].try_into().expect("chroma bytes");
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
