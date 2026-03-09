use edid_info::edid::cta::{AudioFormat, BlockTag, Cta};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_audio_block_acer_ek221q_h() {
    let raw: &[u8; 128] = ACER[128..256].try_into().expect("cta bytes");
    let out = Cta::parse(raw).expect("cta parse");

    let blocks: Vec<_> = out.data_blocks().collect();
    let audio_block = blocks
        .iter()
        .find(|b| b.tag() == BlockTag::Audio)
        .expect("audio block");

    let sad = audio_block.sad(0).expect("sad");
    assert_eq!(audio_block.sads().count(), 1);
    assert_eq!(sad.format(), AudioFormat::Lpcm);
    assert_eq!(sad.channels(), 2);
    assert!(sad.has_rate(32));
    assert!(sad.has_rate(44));
    assert!(sad.has_rate(48));
    assert_eq!(sad.lpcm_depth(), 0b111);
}

#[test]
fn parse_audio_block_asus_rog_pg27u() {
    let raw: &[u8; 128] = ASUS[128..256].try_into().expect("cta bytes");
    let out = Cta::parse(raw).expect("cta parse");

    let blocks: Vec<_> = out.data_blocks().collect();
    let audio_block = blocks
        .iter()
        .find(|b| b.tag() == BlockTag::Audio)
        .expect("audio block");

    let sad = audio_block.sad(0).expect("sad");
    assert_eq!(audio_block.sads().count(), 1);
    assert_eq!(sad.format(), AudioFormat::Lpcm);
    assert_eq!(sad.channels(), 2);
    assert!(sad.has_rate(32));
    assert!(sad.has_rate(44));
    assert!(sad.has_rate(48));
    assert_eq!(sad.lpcm_depth(), 1);
}
