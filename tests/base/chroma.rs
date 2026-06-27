use edid_info::base::chroma::Chroma;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_chroma_acer_ek221q_h() {
    let raw = core::array::from_fn(|i| ACER[i]);
    let out = Chroma::new(&raw);

    assert!((out.red().x() - 662.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.red().y() - 355.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.green().x() - 330.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.green().y() - 645.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.blue().x() - 159.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.blue().y() - 62.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.white().x() - 321.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.white().y() - 337.0_f32 / 1024.0).abs() < f32::EPSILON);
}

#[test]
fn parse_chroma_asus_rog_pg27u() {
    let raw = core::array::from_fn(|i| ASUS[i]);
    let out = Chroma::new(&raw);

    assert!((out.red().x() - 690.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.red().y() - 322.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.green().x() - 198.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.green().y() - 717.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.blue().x() - 154.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.blue().y() - 49.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.white().x() - 320.0_f32 / 1024.0).abs() < f32::EPSILON);
    assert!((out.white().y() - 337.0_f32 / 1024.0).abs() < f32::EPSILON);
}
