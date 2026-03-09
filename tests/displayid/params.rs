use edid_info::edid::displayid::params::DisplayParams;

#[test]
fn test_display_params_parse() {
    let data = [
        0x01, 0x02, // H size 0x0201 = 513
        0x03, 0x04, // V size 0x0403 = 1027
        0x05, 0x06, // H pixels 0x0605 = 1541
        0x07, 0x08, // V pixels 0x0807 = 2055
        0x09, // Features
        0x0A, 0x0B, 0x0C, // Red X, Y
        0x0D, 0x0E, 0x0F, // Green X, Y
        0x10, 0x11, 0x12, // Blue X, Y
        0x13, 0x14, 0x15, // White X, Y
        0x16, 0x17, // Max Lum Full
        0x18, 0x19, // Max Lum 10
        0x1A, 0x1B, // Min Lum
        0x02, // Depth tech (0x02 = 8 bpc)
        0x78, // Gamma (120 = 2.2)
    ];

    let params = DisplayParams::parse(&data).expect("should parse");
    assert_eq!(params.h_size_raw(), 513);
    assert_eq!(params.v_size_raw(), 1027);
    assert_eq!(params.h_pixels(), 1541);
    assert_eq!(params.v_pixels(), 2055);
    assert_eq!(params.color_depth_bpc(), 8);
    assert!((params.gamma().expect("has gamma") - 2.2).abs() < 1e-6);
}

#[test]
fn test_display_params_parse_short() {
    let data = [0; 28];
    assert!(DisplayParams::parse(&data).is_none());
}

#[test]
fn test_display_params_no_gamma() {
    let mut data = [0; 29];
    data[28] = 255;
    let params = DisplayParams::parse(&data).expect("should parse");
    assert_eq!(params.gamma(), None);
}
