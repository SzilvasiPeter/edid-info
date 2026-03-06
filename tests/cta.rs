use edid_info::edid::cta::{AudioFormat, BlockTag, Cta, Speaker};

const EDID: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_cta_acer_ek221q_h() {
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
    assert_eq!(blocks[1].vendor_oui(), Some(0x000_0c03));

    let hdmi = blocks[1].hdmi_vsdb().expect("hdmi vsdb");
    assert_eq!(hdmi.oui(), 0x000_0c03);
    assert_eq!(hdmi.phys_addr(), (1, 0, 0, 0));
    assert!(!hdmi.ai());
    assert!(!hdmi.dc_48());
    assert!(hdmi.dc_36());
    assert!(hdmi.dc_30());
    assert!(hdmi.dc_444());
    assert!(!hdmi.dvi_dual());
    assert_eq!(hdmi.max_tmds_mhz(), Some(250));
    assert!(!hdmi.lat_present());
    assert!(!hdmi.ilat_present());
    assert_eq!(hdmi.video_lat_ms(), None);
    assert_eq!(hdmi.audio_lat_ms(), None);
    assert_eq!(blocks[2].tag(), BlockTag::Extended);
    assert_eq!(blocks[2].ext_tag(), Some(0x05));
    assert_eq!(blocks[3].tag(), BlockTag::Vendor);
    assert_eq!(blocks[3].vendor_oui(), Some(0x000_001a));
    assert_eq!(blocks[3].hdmi_vsdb(), None);
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
fn parse_cta_room_config() {
    let mut raw = [0u8; 128];
    raw[0] = 0x02;
    raw[1] = 3;
    raw[2] = 17;
    raw[4] = 0xEC;
    raw[5] = 13;
    raw[6] = 1;
    raw[7] = 0b1110_0010;
    raw[8] = 0b0000_0011;
    raw[9] = 0b0000_1001;
    raw[10] = 0b0000_0101;
    raw[11] = 10;
    raw[12] = 20;
    raw[13] = 30;
    raw[14] = 40;
    raw[15] = 50;
    raw[16] = 60;

    let out = Cta::parse(&raw).expect("cta parse");
    let block = out.data_blocks().next().expect("block");
    assert_eq!(block.tag(), BlockTag::Extended);
    assert_eq!(block.ext_tag(), Some(13));

    let room = block.room_config().expect("room config");
    assert_eq!(room.rev(), 1);
    assert!(room.cfg().display_valid());
    assert!(room.cfg().speaker_count_valid());
    assert!(room.cfg().sld_present());
    assert_eq!(room.cfg().speaker_count(), Some(3));
    assert!(room.has(Speaker::FlFr));
    assert!(room.has(Speaker::Lfe));
    assert!(room.has(Speaker::TpflTpfr));
    assert!(room.has(Speaker::LsRs));
    assert!(room.has(Speaker::TpblTpbr));
    assert!(room.has(Speaker::BtflBtfr));
    assert_eq!(room.far_x(), Some(10));
    assert_eq!(room.far_y(), Some(20));
    assert_eq!(room.far_z(), Some(30));
    assert_eq!(room.disp_x(), Some(40));
    assert_eq!(room.disp_y(), Some(50));
    assert_eq!(room.disp_z(), Some(60));
}
