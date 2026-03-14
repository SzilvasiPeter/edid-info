use edid_info::edid::header::Header;

const ACER: &[u8] = include_bytes!("data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("data/ASUS_ROG_PG27U.edid");
const CM: &[u8] = include_bytes!("data/CM__CM2400T.edid");
const CS: &[u8] = include_bytes!("data/CS__1920x1080.edid");
const MS: &[u8] = include_bytes!("data/MS__HSD_1903-A00.edid");
const TK: &[u8] = include_bytes!("data/TK@_tianma.edid");
const WG: &[u8] = include_bytes!("data/WG@_UNKNOWN.edid");

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

#[test]
fn parse_header_cm_cm2400t() {
    let raw: [u8; 20] = std::array::from_fn(|i| CM[i]);
    let out = Header::parse(&raw);

    assert_eq!(
        out.pattern(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(out.manufacturer(), ['C', 'M', '?']);
    assert_eq!(out.product(), 9216);
    assert_eq!(out.serial(), 0x0101_0101);
    assert_eq!(out.week(), 45);
    assert_eq!(out.year(), 2017);
    assert_eq!(out.major(), 1);
    assert_eq!(out.minor(), 3);
}

#[test]
fn parse_header_cs_1920x1080() {
    let raw: [u8; 20] = std::array::from_fn(|i| CS[i]);
    let out = Header::parse(&raw);

    assert_eq!(
        out.pattern(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(out.manufacturer(), ['C', 'S', '?']);
    assert_eq!(out.product(), 21009);
    assert_eq!(out.serial(), 1025);
    assert_eq!(out.week(), 5);
    assert_eq!(out.year(), 2013);
    assert_eq!(out.major(), 1);
    assert_eq!(out.minor(), 4);
}

#[test]
fn parse_header_ms_hsd_1903_a00() {
    let raw: [u8; 20] = std::array::from_fn(|i| MS[i]);
    let out = Header::parse(&raw);

    assert_eq!(
        out.pattern(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(out.manufacturer(), ['M', 'S', '?']);
    assert_eq!(out.product(), 60);
    assert_eq!(out.serial(), 0);
    assert_eq!(out.week(), 20);
    assert_eq!(out.year(), 2021);
    assert_eq!(out.major(), 1);
    assert_eq!(out.minor(), 2);
}

#[test]
fn parse_header_tk_tianma() {
    let raw: [u8; 20] = std::array::from_fn(|i| TK[i]);
    let out = Header::parse(&raw);

    assert_eq!(
        out.pattern(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(out.manufacturer(), ['T', 'K', '?']);
    assert_eq!(out.product(), 8427);
    assert_eq!(out.serial(), 0);
    assert_eq!(out.week(), 31);
    assert_eq!(out.year(), 2018);
    assert_eq!(out.major(), 1);
    assert_eq!(out.minor(), 4);
}

#[test]
fn parse_header_wg_unknown() {
    let raw: [u8; 20] = std::array::from_fn(|i| WG[i]);
    let out = Header::parse(&raw);

    assert_eq!(
        out.pattern(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(out.manufacturer(), ['W', 'G', '?']);
    assert_eq!(out.product(), 0);
    assert_eq!(out.serial(), 0);
    assert_eq!(out.week(), 0);
    assert_eq!(out.year(), 2007);
    assert_eq!(out.major(), 1);
    assert_eq!(out.minor(), 1);
}
