use edid_info::edid::header::Header;

const ACER: &[u8] = include_bytes!("data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_header_acer_ek221q_h() {
    let raw: [u8; 20] = std::array::from_fn(|i| ACER[i]);
    let out = Header::parse(&raw);

    assert_eq!(
        out.pattern(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(out.manufacturer(), ['A', 'C', 'R']);
    assert_eq!(out.product(), 2909);
    assert_eq!(out.serial(), 0x3480_002C);
    assert_eq!(out.week(), 48);
    assert_eq!(out.year(), 2023);
    assert_eq!(out.major(), 1);
    assert_eq!(out.minor(), 3);
}

#[test]
fn parse_header_asus_rog_pg27u() {
    let raw: [u8; 20] = std::array::from_fn(|i| ASUS[i]);
    let out = Header::parse(&raw);

    assert_eq!(
        out.pattern(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(out.manufacturer(), ['A', 'U', 'S']);
    assert_eq!(out.product(), 10148);
    assert_eq!(out.serial(), 0x0001_b5bc);
    assert_eq!(out.week(), 30);
    assert_eq!(out.year(), 2018);
    assert_eq!(out.major(), 1);
    assert_eq!(out.minor(), 4);
}
