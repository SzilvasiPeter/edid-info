use edid_info::edid::cta::{BlockTag, Cta};

const ACER: &[u8] = include_bytes!("../data/acer_ek221q_h.edid");

#[test]
fn parse_video_block_acer_ek221q_h() {
    let raw: &[u8; 128] = ACER[128..256].try_into().expect("cta bytes");
    let out = Cta::parse(raw).expect("cta parse");

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
