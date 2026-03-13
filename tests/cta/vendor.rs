use edid_info::edid::cta::{BlockTag, Cta};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_vendor_block_acer_ek221q_h() {
    let raw: [u8; 128] = std::array::from_fn(|i| ACER[128 + i]);
    let out = Cta::parse(&raw).expect("cta parse");

    let blocks: Vec<_> = out.data_blocks().collect();
    let vendor_block = blocks
        .iter()
        .find(|b| b.tag() == BlockTag::Vendor && b.vendor_oui() == Some(0x000_0c03))
        .expect("hdmi vendor block");

    let hdmi = vendor_block.hdmi_vsdb().expect("hdmi vsdb");
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
}

#[test]
fn parse_vendor_block_asus_rog_pg27u() {
    let raw: [u8; 128] = std::array::from_fn(|i| ASUS[128 + i]);
    let out = Cta::parse(&raw).expect("cta parse");

    let blocks: Vec<_> = out.data_blocks().collect();
    let vendor_block = blocks
        .iter()
        .find(|b| b.tag() == BlockTag::Vendor && b.vendor_oui() == Some(0x0000_044b))
        .expect("vendor block");

    assert_eq!(vendor_block.vendor_oui(), Some(0x0000_044b));
    assert_eq!(vendor_block.hdmi_vsdb(), None);
}

#[test]
fn parse_vendor_block_acer_ek221q_h_second() {
    let raw: [u8; 128] = std::array::from_fn(|i| ACER[128 + i]);
    let out = Cta::parse(&raw).expect("cta parse");

    let blocks: Vec<_> = out.data_blocks().collect();
    let vendor_block = blocks
        .iter()
        .find(|b| b.tag() == BlockTag::Vendor && b.vendor_oui() == Some(0x000_001a))
        .expect("second vendor block");

    assert_eq!(vendor_block.vendor_oui(), Some(0x000_001a));
    assert_eq!(vendor_block.hdmi_vsdb(), None);
}
