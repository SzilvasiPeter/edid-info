use edid_info::edid::cta::{BlockTag, Cta, Speaker};

const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_speaker_block_asus_rog_pg27u() {
    let raw: &[u8; 128] = ASUS[128..256].try_into().expect("cta bytes");
    let out = Cta::parse(raw).expect("cta parse");

    let blocks: Vec<_> = out.data_blocks().collect();
    let speaker_block = blocks
        .iter()
        .find(|b| b.tag() == BlockTag::Speaker)
        .expect("speaker block");

    let spk = speaker_block.speaker_alloc().expect("speaker alloc");
    assert!(spk.has(Speaker::FlFr));
    assert!(!spk.has(Speaker::Lfe));
    assert!(!spk.has(Speaker::Fc));
    assert_eq!(spk.bytes(), (0x01, 0x00, 0x00));
}
