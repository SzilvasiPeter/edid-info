use edid_info::edid::cta::{BlockTag, Cta, Vic};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_vics_acer_ek221q_h() {
    let raw: [u8; 128] = std::array::from_fn(|i| ACER[128 + i]);
    let out = Cta::parse(&raw).expect("cta parse");

    let video_block = out
        .data_blocks()
        .find(|b| b.tag() == BlockTag::Video)
        .expect("video block");

    let vic_all: Vec<_> = video_block.svds().map(|svd| svd.vic()).collect();

    assert_eq!(vic_all, vec![16, 1, 3, 4, 18, 19, 31]);

    // Test specific VIC details for some of them
    // VIC 16: 1080p @ 60Hz
    let v16 = Vic::from_vic(16).expect("vic 16");
    assert_eq!(v16.name(), "1080p");
    assert_eq!(v16.width(), 1920);
    assert_eq!(v16.height(), 1080);
    assert_eq!(v16.vfreq_hz(), 60);

    // VIC 1: 640x480 @ 60Hz
    let v1 = Vic::from_vic(1).expect("vic 1");
    assert_eq!(v1.name(), "DMT0659");
    assert_eq!(v1.width(), 640);
    assert_eq!(v1.height(), 480);

    // VIC 31: 1080p @ 50Hz
    let v31 = Vic::from_vic(31).expect("vic 31");
    assert_eq!(v31.name(), "1080p50");
    assert_eq!(v31.vfreq_hz(), 50);
}

#[test]
fn parse_vics_asus_rog_pg27u() {
    // ASUS ROG PG27U CTA extensions (Block 1 and Block 4) do not contain Video Data Blocks (VICs)
    let raw1: [u8; 128] = std::array::from_fn(|i| ASUS[128 + i]);
    let cta1 = Cta::parse(&raw1).expect("cta parse block 1");
    assert!(cta1.data_blocks().all(|b| b.tag() != BlockTag::Video));

    let raw4: [u8; 128] = std::array::from_fn(|i| ASUS[512 + i]);
    let cta4 = Cta::parse(&raw4).expect("cta parse block 4");
    assert!(cta4.data_blocks().all(|b| b.tag() != BlockTag::Video));
}

#[test]
fn test_vic_lookup_bounds() {
    assert!(Vic::from_vic(0).is_none());
    assert!(Vic::from_vic(1).is_some());
    assert!(Vic::from_vic(127).is_some());
    assert!(Vic::from_vic(128).is_none());
    assert!(Vic::from_vic(192).is_none());
    assert!(Vic::from_vic(193).is_some());
    assert!(Vic::from_vic(219).is_some());
    assert!(Vic::from_vic(220).is_none());
}
