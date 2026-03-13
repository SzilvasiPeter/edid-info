use edid_info::edid::cta::header::Header;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_cta_header_acer_ek221q_h() {
    let raw: [u8; 128] = std::array::from_fn(|i| ACER[128 + i]);
    let header = Header::parse(&raw).expect("cta header parse");

    assert_eq!(header.tag(), 0x02);
    assert_eq!(header.rev(), 3);
    assert!(header.underscan());
    assert!(header.basic_audio());
    assert!(header.ycbcr_444());
    assert!(header.ycbcr_422());
    assert_eq!(header.native_dtd_num(), 1);

    // Check offsets
    let dtd_off = header.dtd_off();
    assert!(dtd_off >= 4);
}

#[test]
fn parse_cta_header_asus_rog_pg27u_block_1() {
    let raw: [u8; 128] = std::array::from_fn(|i| ASUS[128 + i]);
    let header = Header::parse(&raw).expect("cta header parse block 1");

    assert_eq!(header.tag(), 0x02);
    assert_eq!(header.rev(), 3);
    assert!(header.underscan());
    assert!(header.basic_audio());
    assert!(header.ycbcr_444());
    assert!(header.ycbcr_422());
    assert_eq!(header.native_dtd_num(), 1);
}

#[test]
fn parse_cta_header_asus_rog_pg27u_block_4() {
    let raw: [u8; 128] = std::array::from_fn(|i| ASUS[512 + i]);
    let header = Header::parse(&raw).expect("cta header parse block 4");

    assert_eq!(header.tag(), 0x02);
    assert_eq!(header.rev(), 3);
    assert_eq!(header.native_dtd_num(), 1);
}

#[test]
fn parse_cta_header_invalid() {
    let mut raw = [0u8; 128];
    raw[0] = 0x00; // Not CTA_TAG
    assert!(Header::parse(&raw).is_none());
}
