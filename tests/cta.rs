use edid_info::edid::cta::{BlockTag, Cta, CTA_LEN};

const EDID: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_cta_happy_path_from_real_edid() {
    assert_eq!(EDID.len(), 256);
    let raw: &[u8; 128] = EDID[128..256].try_into().expect("cta bytes");
    let out = Cta::parse(raw).expect("cta parse");

    assert_eq!(out.rev(), 3);
    assert_eq!(out.native_dtd_num(), 1);
    assert!(out.underscan());
    assert!(out.basic_audio());
    assert!(out.ycbcr_444());
    assert!(out.ycbcr_422());

    let blocks: Vec<_> = out.data_blocks().collect();
    assert_eq!(blocks.len(), 5);
    assert_eq!(blocks[0].tag(), BlockTag::Video);
    assert_eq!(blocks[1].tag(), BlockTag::Vendor);
    assert_eq!(blocks[2].tag(), BlockTag::Extended);
    assert_eq!(blocks[2].ext_tag(), Some(0x05));
    assert_eq!(blocks[3].tag(), BlockTag::Vendor);
    assert_eq!(blocks[4].tag(), BlockTag::Audio);
    assert_eq!(
        blocks[0].svds().map(|svd| (svd.vic(), svd.native())).collect::<Vec<_>>(),
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

    let dtd0 = out.dtd(0).expect("cta dtd 0");
    assert_eq!(dtd0.pixel_clock_hz(), 174_500_000);
    assert_eq!(dtd0.h_active(), 1920);
    assert_eq!(dtd0.v_active(), 1080);

    let dtd1 = out.dtd(1).expect("cta dtd 1");
    assert_eq!(dtd1.pixel_clock_hz(), 228_800_000);
    assert_eq!(dtd1.h_active(), 1920);
    assert_eq!(dtd1.v_active(), 1080);

    assert!(out.dtd(2).is_none());
    assert_eq!(out.checksum(), 0x92);
    assert!(out.checksum_ok());
}

#[test]
fn parse_video_svd_vic_8bit() {
    let mut raw = [0u8; CTA_LEN];
    raw[0] = 0x02;
    raw[2] = 6;
    raw[4] = 0x41;
    raw[5] = 0xC1;

    let cta = Cta::parse(&raw).expect("cta parse");
    let block = cta.data_blocks().next().expect("data block");
    let svd = block.svd(0).expect("svd");
    assert_eq!(svd.vic(), 193);
    assert!(!svd.native());
}
