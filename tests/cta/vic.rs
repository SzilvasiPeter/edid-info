use edid_info::edid::cta::{BlockTag, Cta, Vic};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");
const ACR_XG270HU: &[u8] = include_bytes!("../data/ACR_XG270HU.edid");
const GSM_LG_TV: &[u8] = include_bytes!("../data/GSM_LG_TV_SSCR2.edid");

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

    let v16 = Vic::from_vic(16).expect("vic 16");
    assert_eq!(v16.name(), "1080p");
    assert_eq!(v16.width(), 1920);
    assert_eq!(v16.height(), 1080);
    assert_eq!(v16.vfreq_hz(), 60);

    let v1 = Vic::from_vic(1).expect("vic 1");
    assert_eq!(v1.name(), "DMT0659");
    assert_eq!(v1.width(), 640);
    assert_eq!(v1.height(), 480);

    let v31 = Vic::from_vic(31).expect("vic 31");
    assert_eq!(v31.name(), "1080p50");
    assert_eq!(v31.vfreq_hz(), 50);
}

#[test]
fn parse_vics_asus_rog_pg27u() {
    let raw1: [u8; 128] = std::array::from_fn(|i| ASUS[128 + i]);
    let cta1 = Cta::parse(&raw1).expect("cta parse block 1");
    assert!(cta1.data_blocks().all(|b| b.tag() != BlockTag::Video));

    let raw4: [u8; 128] = std::array::from_fn(|i| ASUS[512 + i]);
    let cta4 = Cta::parse(&raw4).expect("cta parse block 4");
    assert!(cta4.data_blocks().all(|b| b.tag() != BlockTag::Video));
}

#[test]
fn parse_vics_acr_xg270hu() {
    let raw: [u8; 128] = std::array::from_fn(|i| ACR_XG270HU[128 + i]);
    let out = Cta::parse(&raw).expect("cta parse");

    let video_block = out
        .data_blocks()
        .find(|b| b.tag() == BlockTag::Video)
        .expect("video block");
    assert!(video_block.svds().any(|svd| svd.vic() == 90), "no VIC 90");

    let v90 = Vic::from_vic(90).expect("vic 90");
    assert_eq!(v90.name(), "1080p2x");
    assert_eq!(v90.width(), 2560);
    assert_eq!(v90.height(), 1080);
    assert_eq!(v90.vfreq_hz(), 60);
}

#[test]
fn parse_vics_gsm_lg_tv_sscr2() {
    let raw: [u8; 128] = std::array::from_fn(|i| GSM_LG_TV[128 + i]);
    let out = Cta::parse(&raw).expect("cta parse");

    let video_block = out
        .data_blocks()
        .find(|b| b.tag() == BlockTag::Video)
        .expect("video block");

    assert!(video_block.svds().any(|svd| svd.vic() == 63), "no VIC 63");
    assert!(video_block.svds().any(|svd| svd.vic() == 64), "no VIC 64");
    assert!(video_block.svds().any(|svd| svd.vic() == 93), "no VIC 93");
    assert!(video_block.svds().any(|svd| svd.vic() == 94), "no VIC 94");
    assert!(video_block.svds().any(|svd| svd.vic() == 95), "no VIC 95");
    assert!(video_block.svds().any(|svd| svd.vic() == 98), "no VIC 98");
    assert!(video_block.svds().any(|svd| svd.vic() == 99), "no VIC 99");
    assert!(video_block.svds().any(|svd| svd.vic() == 100), "no VIC 100");
    assert!(video_block.svds().any(|svd| svd.vic() == 117), "no VIC 117");
    assert!(video_block.svds().any(|svd| svd.vic() == 118), "no VIC 118");
    assert!(video_block.svds().any(|svd| svd.vic() == 218), "no VIC 218");
    assert!(video_block.svds().any(|svd| svd.vic() == 219), "no VIC 219");

    let v63 = Vic::from_vic(63).expect("vic 63");
    assert_eq!(v63.name(), "1080p120");
    assert_eq!(v63.width(), 1920);
    assert_eq!(v63.height(), 1080);
    assert_eq!(v63.vfreq_hz(), 120);

    let v93 = Vic::from_vic(93).expect("vic 93");
    assert_eq!(v93.name(), "2160p24");
    assert_eq!(v93.width(), 3840);
    assert_eq!(v93.height(), 2160);
    assert_eq!(v93.vfreq_hz(), 24);

    let v99 = Vic::from_vic(99).expect("vic 99");
    assert_eq!(v99.name(), "2160p25");
    assert_eq!(v99.width(), 4096);
    assert_eq!(v99.height(), 2160);
    assert_eq!(v99.vfreq_hz(), 25);

    let v118 = Vic::from_vic(118).expect("vic 118");
    assert_eq!(v118.name(), "2160p120");
    assert_eq!(v118.width(), 3840);
    assert_eq!(v118.height(), 2160);
    assert_eq!(v118.vfreq_hz(), 120);

    let v219 = Vic::from_vic(219).expect("vic 219");
    assert_eq!(v219.name(), "2160p120");
    assert_eq!(v219.width(), 4096);
    assert_eq!(v219.height(), 2160);
    assert_eq!(v219.vfreq_hz(), 120);
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
