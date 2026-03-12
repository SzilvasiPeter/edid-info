use edid_info::edid::cta::{BlockTag, Cta, Speaker};

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
    let sum: u16 = raw[..127].iter().map(|&b| u16::from(b)).sum();
    let chk = (256u16 - (sum % 256)) % 256;
    raw[127] = u8::try_from(chk).expect("checksum byte");

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
