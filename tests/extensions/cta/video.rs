use edid_info::extensions::cta::{Cta, block::BlockTag};

const ACER: &[u8] = include_bytes!("../../data/ACER_EK221Q_H.edid");
const ACR_XG270HU: &[u8] = include_bytes!("../../data/ACR_XG270HU.edid");
const GSM_LG_TV: &[u8] = include_bytes!("../../data/GSM_LG_TV_SSCR2.edid");

#[test]
fn parse_video_block_acer_ek221q_h() {
    let raw: [u8; 128] = std::array::from_fn(|i| ACER[128 + i]);
    let out = Cta::parse(&raw).expect("cta parse");

    let blocks: Vec<_> = out.data_blocks().collect();
    let video_block = blocks
        .iter()
        .find(|b| b.tag() == BlockTag::Video)
        .expect("video block");

    let svds: Vec<_> = video_block
        .svds()
        .map(|svd| (svd.vic(), svd.native()))
        .collect();

    assert_eq!(
        svds,
        vec![
            (16, true),
            (1, false),
            (3, false),
            (4, false),
            (18, false),
            (19, false),
            (31, true),
        ]
    );
}

#[test]
fn parse_video_block_acr_xg270hu() {
    let raw: [u8; 128] = std::array::from_fn(|i| ACR_XG270HU[128 + i]);
    let out = Cta::parse(&raw).expect("cta parse");

    let video_block = out
        .data_blocks()
        .find(|b| b.tag() == BlockTag::Video)
        .expect("video block");

    let svds: Vec<_> = video_block
        .svds()
        .map(|svd| (svd.vic(), svd.native()))
        .collect();

    assert_eq!(
        svds,
        vec![
            (16, true),
            (5, false),
            (4, false),
            (3, false),
            (2, false),
            (1, false),
            (17, false),
            (18, false),
            (19, false),
            (20, false),
            (6, false),
            (7, false),
            (21, false),
            (22, false),
            (31, false),
            (32, false),
            (90, false),
        ]
    );
}

#[test]
fn parse_video_block_gsm_lg_tv_sscr2() {
    let raw: [u8; 128] = std::array::from_fn(|i| GSM_LG_TV[128 + i]);
    let out = Cta::parse(&raw).expect("cta parse");

    let video_block = out
        .data_blocks()
        .find(|b| b.tag() == BlockTag::Video)
        .expect("video block");

    let svds: Vec<_> = video_block
        .svds()
        .map(|svd| (svd.vic(), svd.native()))
        .collect();

    assert_eq!(
        svds,
        vec![
            (118, false),
            (118, false),
            (118, false),
            (117, false),
            (118, false),
            (118, false),
            (219, false),
            (218, false),
            (16, false),
            (31, false),
            (4, false),
            (19, false),
            (5, false),
            (20, false),
            (3, false),
            (2, false),
            (18, false),
            (32, false),
            (33, false),
            (34, false),
            (21, false),
            (1, false),
            (93, false),
            (94, false),
            (95, false),
            (98, false),
            (99, false),
            (100, false),
            (63, false),
            (64, false),
        ]
    );
}
