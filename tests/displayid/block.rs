use edid_info::edid::BLOCK_LEN;
use edid_info::edid::displayid::DisplayId;
use edid_info::edid::displayid::block::BlockTag;

#[test]
fn test_block_tag_parse() {
    assert_eq!(BlockTag::parse(0x20), BlockTag::ProductIdent);
    assert_eq!(BlockTag::parse(0x21), BlockTag::DisplayParams);
    assert_eq!(BlockTag::parse(0x22), BlockTag::TimingV7);
    assert_eq!(BlockTag::parse(0x26), BlockTag::InterfaceFeatures);
    assert_eq!(BlockTag::parse(0x7E), BlockTag::VendorSpecific);
    assert_eq!(BlockTag::parse(0x81), BlockTag::CtaDisplayId);
    assert_eq!(BlockTag::parse(0xFF), BlockTag::Reserved(0xFF));
}

#[test]
fn test_data_block_iter() {
    let mut raw = [0u8; BLOCK_LEN];
    raw[0] = 0x70;
    raw[1] = 0x20;
    raw[2] = 0x20; // Version
    raw[3] = 15; // Section length (3 header + 12 data)
    raw[4] = 0;
    raw[5] = 0;
    raw[6] = 0x20; // Tag ProductIdent
    raw[7] = 0; // Rev
    raw[8] = 12; // Len
    // Payload for ProductIdent (12 bytes)
    raw[9] = 0x01;
    raw[10] = 0x02;
    raw[11] = 0x03; // OUI
    raw[12] = 0x04;
    raw[13] = 0x05; // Product
    raw[14] = 0x06;
    raw[15] = 0x07;
    raw[16] = 0x08;
    raw[17] = 0x09; // Serial
    raw[18] = 0x0A; // Week
    raw[19] = 0x0B; // Year
    raw[20] = 0x00; // Name len

    // Checksum
    let mut sum = 0u8;
    for &b in &raw[..127] {
        sum = sum.wrapping_add(b);
    }
    raw[127] = 0u8.wrapping_sub(sum);

    let did = DisplayId::parse(&raw).expect("should parse");
    let mut blocks = did.data_blocks();
    let block = blocks.next().expect("has block");
    assert_eq!(block.tag(), BlockTag::ProductIdent);
    let ident = block.product_ident().expect("is product ident");
    assert_eq!(ident.oui(), [0x01, 0x02, 0x03]);
    assert_eq!(ident.year(), 2011);

    assert!(blocks.next().is_none());
}
