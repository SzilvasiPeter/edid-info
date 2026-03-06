use edid_info::edid::header::Header;

const EDID: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_header_acer_ek221q_h() {
    let raw: &[u8; 128] = EDID[0..128].try_into().expect("base block bytes");
    let out = Header::parse(raw);

    assert_eq!(
        out.magic(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(out.maker(), ['A', 'C', 'R']);
    assert_eq!(out.product(), 2909);
    assert_eq!(out.serial(), 0x3480_002C);
    assert_eq!(out.week(), 48);
    assert_eq!(out.year(), 2023);
    assert_eq!(out.major(), 1);
    assert_eq!(out.minor(), 3);
}
