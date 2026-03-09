use edid_info::edid::displayid::interface::InterfaceFeatures;

#[test]
fn test_interface_features_parse() {
    let data = [
        0x01, 0x02, 0x03, 0x04, // BPC
        0x00, 0x00, // Reserved
        0x61, // EOTFs (0x61 = sRGB, BT2020, BT2020 ST2084)
        0x00, 0x00, // Reserved
    ];

    let iface = InterfaceFeatures::parse(&data).expect("should parse");
    assert_eq!(iface.bpc_rgb(), 0x01);
    assert_eq!(iface.bpc_y444(), 0x02);
    assert_eq!(iface.bpc_y422(), 0x03);
    assert_eq!(iface.bpc_y420(), 0x04);
    assert!(iface.srgb_eotf());
    assert!(iface.bt2020_eotf());
    assert!(iface.bt2020_st2084_eotf());
}

#[test]
fn test_interface_features_parse_short() {
    let data = [0; 8];
    assert!(InterfaceFeatures::parse(&data).is_none());
}
