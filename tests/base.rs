use edid_info::edid::base::BaseEdid;
use edid_info::edid::cta::{BlockTag, Cta, Speaker};
use edid_info::edid::descriptor::monitor::DescTag;
use edid_info::edid::dtd::Mode;

const ACER: &[u8] = include_bytes!("data/acer_ek221q_h.edid");

#[test]
fn parse_base_acer_ek221q_h() {
    assert_eq!(ACER.len(), 256);

    let raw: &[u8; 128] = ACER[0..128].try_into().expect("base block bytes");
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

const ASUS: &[u8] = include_bytes!("data/asus_rog_pg27u.edid");

#[test]
fn parse_base_asus_rog_pg27u() {
    assert_eq!(ASUS.len(), 768);

    let raw_base: &[u8; 128] = ASUS[0..128].try_into().expect("base block");
    let base = BaseEdid::parse(raw_base);
    assert_eq!(
        base.header().magic(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(base.header().maker(), ['A', 'U', 'S']);
    assert_eq!(base.header().product(), 10148);
    assert_eq!(base.header().serial(), 0x0001_b5bc);
    assert_eq!(base.header().week(), 30);
    assert_eq!(base.header().year(), 2018);
    assert_eq!(base.header().major(), 1);
    assert_eq!(base.header().minor(), 4);

    assert_eq!(base.basic().width_cm(), 60);
    assert_eq!(base.basic().height_cm(), 34);
    assert_eq!(base.basic().gamma_raw(), 120);

    assert_eq!(base.footer().extension_num(), 2);
    assert_eq!(base.footer().checksum(), 0x72);
    assert!(base.checksum_ok());

    let raw_cta: &[u8; 128] = ASUS[128..256].try_into().expect("cta block");
    let cta = Cta::parse(raw_cta).expect("cta parse");

    assert_eq!(cta.rev(), 3);
    assert_eq!(cta.native_dtd_num(), 1);
    assert!(cta.underscan());
    assert!(cta.basic_audio());
    assert!(cta.ycbcr_444());
    assert!(cta.ycbcr_422());
    assert_eq!(cta.checksum(), 0x46);
    assert!(cta.checksum_ok());

    let blocks: Vec<_> = cta.data_blocks().collect();
    assert_eq!(blocks.len(), 6);
    assert_eq!(blocks[0].tag(), BlockTag::Audio);
    assert_eq!(blocks[1].tag(), BlockTag::Speaker);
    let spk = blocks[1].speaker_alloc().expect("speaker alloc");
    assert!(spk.has(Speaker::FlFr));
    assert!(!spk.has(Speaker::Lfe));
    assert!(!spk.has(Speaker::Fc));
    assert_eq!(spk.bytes(), (0x01, 0x00, 0x00));
    assert_eq!(blocks[2].tag(), BlockTag::Vendor);
    assert_eq!(blocks[2].vendor_oui(), Some(0x0000_044b));

    let dtd0 = cta.dtd(0).expect("cta dtd 0");
    assert_eq!(dtd0.pixel_clock_hz(), 262_750_000);
    assert_eq!(dtd0.h_active(), 3840);
    assert_eq!(dtd0.v_active(), 2160);

    let dtd1 = cta.dtd(1).expect("cta dtd 1");
    assert_eq!(dtd1.pixel_clock_hz(), 209_750_000);
    assert_eq!(dtd1.h_active(), 3840);
    assert_eq!(dtd1.v_active(), 2160);
}
