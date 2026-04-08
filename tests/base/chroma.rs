use edid_info::base::chroma::Chroma;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

fn base(raw: &[u8]) -> [u8; 128] {
    core::array::from_fn(|i| raw[i])
}

#[test]
fn parse_chroma_acer_ek221q_h() {
    let raw = base(ACER);
    let out = Chroma::new(&raw);

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
    let raw = base(ASUS);
    let out = Chroma::new(&raw);

    assert_eq!(out.red().x(), 690);
    assert_eq!(out.red().y(), 322);
    assert_eq!(out.green().x(), 198);
    assert_eq!(out.green().y(), 717);
    assert_eq!(out.blue().x(), 154);
    assert_eq!(out.blue().y(), 49);
    assert_eq!(out.white().x(), 320);
    assert_eq!(out.white().y(), 337);
}
