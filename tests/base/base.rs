use edid_info::{
    base::Base,
    base::basic::ScreenSize,
    base::descriptor::monitor::DescTag,
    base::descriptors::Descriptor,
    base::established::EstablishedTiming,
    base::header::DateInfo,
    common::Size,
    common::Version,
    extensions::{
        Extension,
        cta::{block::BlockTag, speaker::Speaker},
    },
};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_base_acer_ek221q_h() {
    assert_eq!(ACER.len(), 256);

    let raw: [u8; 128] = std::array::from_fn(|i| ACER[i]);
    let out = Base::new(&raw);
    assert_eq!(out.header().manufacturer(), ['A', 'C', 'R']);
    assert_eq!(
        out.basic().screen_size(),
        ScreenSize::Dimensions(Size::new(480, 260))
    );
    assert_eq!(out.chroma().white().x(), 321);
    assert!(
        out.established()
            .supported()
            .contains(&Some(EstablishedTiming::T1280x1024_75))
    );
    assert_eq!(
        out.timings()
            .mode(7)
            .map(|m| (m.width(), m.height(), m.vfreq())),
        Some((1920, 1080, 75))
    );

    match out
        .descriptors()
        .descriptors(1)
        .expect("dtd mode 1 should exist")
    {
        Descriptor::Timing(timing) => {
            let h = timing.horizontal();
            let v = timing.vertical();
            assert_eq!(h.active(), 1920);
            assert_eq!(v.active(), 1080);
            assert_eq!(timing.pixel_clock_khz(), 148_500);
        }
        Descriptor::Display(serial) => {
            assert_eq!(serial.tag(), DescTag::SerialNumber);
            assert_eq!(serial.serial(), Some("13480002C3W01"));
        }
    }

    assert_eq!(out.footer().extension_count(), 1);
    assert_eq!(out.footer().checksum(), 0x18);
}

#[test]
fn parse_base_asus_rog_pg27u() {
    assert_eq!(ASUS.len(), 768);

    let raw_base: [u8; 128] = std::array::from_fn(|i| ASUS[i]);
    let base = Base::new(&raw_base);
    assert_eq!(
        base.header().pattern(),
        [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]
    );
    assert_eq!(base.header().manufacturer(), ['A', 'U', 'S']);
    assert_eq!(base.header().product(), 10148);
    assert_eq!(base.header().serial(), 0x0001_b5bc);
    assert_eq!(
        base.header().date(),
        DateInfo::Manufacture {
            week: 30,
            year: 2018
        }
    );
    assert_eq!(base.header().version(), Version::new(1, 4));

    assert_eq!(
        base.basic().screen_size(),
        ScreenSize::Dimensions(Size::new(600, 340))
    );
    assert_eq!(base.basic().gamma(), Some(220));

    assert_eq!(base.footer().extension_count(), 2);
    assert_eq!(base.footer().checksum(), 0x72);

    let raw_cta: [u8; 128] = std::array::from_fn(|i| ASUS[128 + i]);
    let Extension::Cta(cta) = Extension::parse(&raw_cta) else {
        panic!("expected CTA extension")
    };

    assert_eq!(cta.revision(), 3);
    assert_eq!(cta.native_dtd_num(), 1);
    assert!(cta.underscan());
    assert!(cta.basic_audio());
    assert!(cta.ycbcr_444());
    assert!(cta.ycbcr_422());
    assert_eq!(cta.checksum(), 0x46);

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
    assert_eq!(dtd0.pixel_clock_khz(), 262_750);
    let h0 = dtd0.horizontal();
    let v0 = dtd0.vertical();
    assert_eq!(h0.active(), 3840);
    assert_eq!(v0.active(), 2160);

    let dtd1 = cta.dtd(1).expect("cta dtd 1");
    assert_eq!(dtd1.pixel_clock_khz(), 209_750);
    let h1 = dtd1.horizontal();
    let v1 = dtd1.vertical();
    assert_eq!(h1.active(), 3840);
    assert_eq!(v1.active(), 2160);
}
