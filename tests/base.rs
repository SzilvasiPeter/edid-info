use edid_info::edid::base::BaseEdid;
use edid_info::edid::descriptor::monitor::DescTag;
use edid_info::edid::dtd::Mode;

const EDID: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_base_happy_path_from_real_edid() {
    assert_eq!(EDID.len(), 256);
    let raw: &[u8; 128] = EDID[0..128].try_into().expect("base block bytes");
    let out = BaseEdid::parse(raw);

    assert_eq!(out.header().maker(), ['A', 'C', 'R']);
    assert_eq!(out.basic().width_cm(), 48);
    assert_eq!(out.chroma().white().x(), 321);
    assert!(out.established().t_1280_1024_75());
    assert_eq!(
        out.standard()
            .mode(7)
            .map(|m| (m.width(), m.height(), m.vfreq())),
        Some((1920, 1080, 75))
    );

    match out.dtd().mode(1).expect("dtd mode 1 should exist") {
        Mode::Timing(timing) => {
            assert_eq!(timing.h_active(), 1920);
            assert_eq!(timing.v_active(), 1080);
            assert_eq!(timing.pixel_clock_hz(), 148_500_000);
        }
        Mode::Display(serial) => {
            assert_eq!(serial.tag(), DescTag::SerialNumber);
            assert_eq!(serial.serial(), Some("13480002C3W01"));
        }
    }

    assert_eq!(out.footer().extension_num(), 1);
    assert_eq!(out.footer().checksum(), 0x18);
    assert!(out.checksum_ok());
}
