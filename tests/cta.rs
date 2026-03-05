use edid_info::edid::cta::{AudioExtFormat, AudioFormat, BlockTag, CTA_LEN, Cta};

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
    assert_eq!(
        blocks[0]
            .svds()
            .map(|svd| (svd.vic(), svd.native()))
            .collect::<Vec<_>>(),
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
    assert_eq!(blocks[1].tag(), BlockTag::Vendor);
    assert_eq!(blocks[2].tag(), BlockTag::Extended);
    assert_eq!(blocks[2].ext_tag(), Some(0x05));
    assert_eq!(blocks[3].tag(), BlockTag::Vendor);
    assert_eq!(blocks[4].tag(), BlockTag::Audio);
    let sad = blocks[4].sad(0).expect("sad");
    assert_eq!(blocks[4].sads().count(), 1);
    assert_eq!(sad.format(), AudioFormat::Lpcm);
    assert_eq!(sad.channels(), 2);
    assert!(sad.has_rate(32));
    assert!(sad.has_rate(44));
    assert!(sad.has_rate(48));
    assert_eq!(sad.lpcm_depth(), 0b111);

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
    raw[0] = 2;
    raw[2] = 6;
    raw[4] = 65;
    raw[5] = 193;

    let cta = Cta::parse(&raw).expect("cta parse");
    let block = cta.data_blocks().next().expect("data block");
    let svd = block.svd(0).expect("svd");
    assert_eq!(svd.vic(), 193);
    assert!(!svd.native());
}

#[test]
fn resolve_common_vic_timings() {
    let mut raw = [0u8; CTA_LEN];
    raw[0] = 2;
    raw[2] = 17;
    raw[4] = 76;
    raw[5] = 129;
    raw[6] = 3;
    raw[7] = 4;
    raw[8] = 144;
    raw[9] = 18;
    raw[10] = 19;
    raw[11] = 159;
    raw[12] = 93;
    raw[13] = 94;
    raw[14] = 95;
    raw[15] = 96;
    raw[16] = 97;

    let cta = Cta::parse(&raw).expect("cta parse");
    let block = cta.data_blocks().next().expect("data block");
    let out = block
        .svds()
        .filter_map(|svd| {
            svd.timing().map(|t| {
                (
                    svd.vic(),
                    t.name(),
                    t.width(),
                    t.height(),
                    t.vfreq_millihz(),
                )
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(
        out,
        vec![
            (1, "DMT0659", 640, 480, 59_940),
            (3, "480pH", 720, 480, 59_940),
            (4, "720p", 1280, 720, 60_000),
            (16, "1080p", 1920, 1080, 60_000),
            (18, "576pH", 720, 576, 50_000),
            (19, "720p50", 1280, 720, 50_000),
            (31, "1080p50", 1920, 1080, 50_000),
            (93, "2160p24", 3840, 2160, 24_000),
            (94, "2160p25", 3840, 2160, 25_000),
            (95, "2160p30", 3840, 2160, 30_000),
            (96, "2160p50", 3840, 2160, 50_000),
            (97, "2160p60", 3840, 2160, 60_000),
        ]
    );
}

#[test]
fn parse_audio_sad_formats() {
    let mut raw = [0u8; CTA_LEN];
    raw[0] = 2;
    raw[2] = 11;
    raw[4] = 38;
    raw[5] = 21;
    raw[6] = 7;
    raw[7] = 80;
    raw[8] = 122;
    raw[9] = 4;
    raw[10] = 99;

    let cta = Cta::parse(&raw).expect("cta parse");
    let block = cta.data_blocks().next().expect("data block");
    assert_eq!(block.tag(), BlockTag::Audio);

    let ac3 = block.sad(0).expect("ac3");
    assert_eq!(ac3.format(), AudioFormat::Ac3);
    assert_eq!(ac3.channels(), 6);
    assert_eq!(ac3.max_kbps(), Some(640));
    assert_eq!(ac3.ext(), None);

    let ext = block.sad(1).expect("ext");
    assert_eq!(ext.format(), AudioFormat::Ext);
    assert_eq!(ext.channels(), 3);
    assert_eq!(ext.ext(), Some(AudioExtFormat::Ac4));
    assert_eq!(ext.max_kbps(), None);
}
