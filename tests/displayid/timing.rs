use edid_info::edid::displayid::timing::TimingV7;

#[test]
fn test_timing_v7_parse() {
    let mut data = [0; 20];
    data[0] = 0x10; // Pixel clock 0x000210 = 528 (Wait, 3 bytes)
    data[1] = 0x02;
    data[2] = 0x00;
    data[3] = 0x80; // Preferred bit
    data[4] = 0x80; // H active 1920
    data[5] = 0x07;
    data[12] = 0x38; // V active 1080
    data[13] = 0x04;

    let timing = TimingV7::parse(1, &data).expect("should parse");
    assert_eq!(timing.rev(), 1);
    let desc = timing.descriptors().next().expect("has descriptor");
    assert_eq!(desc.pixel_clock_khz(), 528);
    assert_eq!(desc.h_active(), 1920);
    assert_eq!(desc.v_active(), 1080);
    assert!(desc.is_preferred());
}

#[test]
fn test_timing_v7_parse_short() {
    let data = [0; 19];
    assert!(TimingV7::parse(1, &data).is_none());
}

#[test]
fn test_timing_v7_parse_unaligned() {
    let data = [0; 25];
    assert!(TimingV7::parse(1, &data).is_none());
}
