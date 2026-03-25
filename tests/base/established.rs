use edid_info::base::established::{Established, EstablishedTiming};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

fn base(raw: &[u8]) -> [u8; 128] {
    core::array::from_fn(|i| raw[i])
}

#[test]
fn parse_established_acer_ek221q_h() {
    let raw = base(ACER);
    let out = Established::new(&raw);
    let list = out.supported();

    assert!(has(&list, EstablishedTiming::T720x400_70));
    assert!(!has(&list, EstablishedTiming::T720x400_88));
    assert!(has(&list, EstablishedTiming::T640x480_60));
    assert!(has(&list, EstablishedTiming::T640x480_67));
    assert!(has(&list, EstablishedTiming::T640x480_72));
    assert!(has(&list, EstablishedTiming::T640x480_75));
    assert!(has(&list, EstablishedTiming::T800x600_56));
    assert!(has(&list, EstablishedTiming::T800x600_60));
    assert!(has(&list, EstablishedTiming::T800x600_72));
    assert!(has(&list, EstablishedTiming::T800x600_75));
    assert!(has(&list, EstablishedTiming::T832x624_75));
    assert!(!has(&list, EstablishedTiming::T1024x768_87I));
    assert!(has(&list, EstablishedTiming::T1024x768_60));
    assert!(has(&list, EstablishedTiming::T1024x768_70));
    assert!(has(&list, EstablishedTiming::T1024x768_75));
    assert!(has(&list, EstablishedTiming::T1280x1024_75));
    assert!(has(&list, EstablishedTiming::T1152x870_75));
    assert_eq!(out.manufacturer_bits(), 0);
}

#[test]
fn parse_established_asus_rog_pg27u() {
    let raw = base(ASUS);
    let out = Established::new(&raw);
    let list = out.supported();

    assert!(!has(&list, EstablishedTiming::T720x400_70));
    assert!(!has(&list, EstablishedTiming::T720x400_88));
    assert!(has(&list, EstablishedTiming::T640x480_60));
    assert!(!has(&list, EstablishedTiming::T640x480_67));
    assert!(!has(&list, EstablishedTiming::T640x480_72));
    assert!(!has(&list, EstablishedTiming::T640x480_75));
    assert!(!has(&list, EstablishedTiming::T800x600_56));
    assert!(has(&list, EstablishedTiming::T800x600_60));
    assert!(!has(&list, EstablishedTiming::T800x600_72));
    assert!(!has(&list, EstablishedTiming::T800x600_75));
    assert!(!has(&list, EstablishedTiming::T832x624_75));
    assert!(!has(&list, EstablishedTiming::T1024x768_87I));
    assert!(has(&list, EstablishedTiming::T1024x768_60));
    assert!(!has(&list, EstablishedTiming::T1024x768_70));
    assert!(!has(&list, EstablishedTiming::T1024x768_75));
    assert!(!has(&list, EstablishedTiming::T1280x1024_75));
    assert!(!has(&list, EstablishedTiming::T1152x870_75));
    assert_eq!(out.manufacturer_bits(), 0);
}

fn has(list: &[Option<EstablishedTiming>; 17], t: EstablishedTiming) -> bool {
    list.contains(&Some(t))
}
