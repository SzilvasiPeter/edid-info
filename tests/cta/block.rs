use edid_info::edid::cta::{BlockTag, Cta};

const ACER: &[u8] = include_bytes!("../data/acer_ek221q_h.edid");
const ASUS: &[u8] = include_bytes!("../data/asus_rog_pg27u.edid");

#[test]
fn parse_cta_acer_ek221q_h() {
    let raw: &[u8; 128] = ACER[128..256].try_into().expect("cta bytes");
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
    assert_eq!(blocks[3].tag(), BlockTag::Vendor);
    assert_eq!(blocks[4].tag(), BlockTag::Audio);

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
fn parse_cta_asus_rog_pg27u() {
    let raw: &[u8; 128] = ASUS[128..256].try_into().expect("cta bytes");
    let out = Cta::parse(raw).expect("cta parse");
    assert_eq!(out.rev(), 3);
    assert_eq!(out.native_dtd_num(), 1);
    assert!(out.underscan());
    assert!(out.basic_audio());
    assert!(out.ycbcr_444());
    assert!(out.ycbcr_422());

    let blocks: Vec<_> = out.data_blocks().collect();
    assert_eq!(blocks.len(), 6);

    assert_eq!(blocks[0].tag(), BlockTag::Audio);
    assert_eq!(blocks[1].tag(), BlockTag::Speaker);
    assert_eq!(blocks[2].tag(), BlockTag::Vendor);
    assert_eq!(blocks[3].tag(), BlockTag::Extended);
    assert_eq!(blocks[4].tag(), BlockTag::Extended);
    assert_eq!(blocks[5].tag(), BlockTag::Extended);

    let dtd0 = out.dtd(0).expect("cta dtd 0");
    assert_eq!(dtd0.pixel_clock_hz(), 262_750_000);
    assert_eq!(dtd0.h_active(), 3840);
    assert_eq!(dtd0.v_active(), 2160);

    let dtd1 = out.dtd(1).expect("cta dtd 1");
    assert_eq!(dtd1.pixel_clock_hz(), 209_750_000);
    assert_eq!(dtd1.h_active(), 3840);
    assert_eq!(dtd1.v_active(), 2160);

    assert!(out.dtd(2).is_none());
    assert_eq!(out.checksum(), 0x46);
    assert!(out.checksum_ok());
}
