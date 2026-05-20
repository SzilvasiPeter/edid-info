use edid_info::base::established::EstablishedLegacy;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

fn base(raw: &[u8]) -> [u8; 128] {
    core::array::from_fn(|i| raw[i])
}

#[test]
fn parse_established_acer_ek221q_h() {
    let raw = base(ACER);
    let out = EstablishedLegacy::new(&raw);
    let list = out.supported();

    assert_id(&list, 0, Some(0x00));
    assert_id(&list, 1, None);
    assert_id(&list, 2, Some(0x04));
    assert_id(&list, 3, Some(0x00));
    assert_id(&list, 4, Some(0x05));
    assert_id(&list, 5, Some(0x06));
    assert_id(&list, 6, Some(0x08));
    assert_id(&list, 7, Some(0x09));
    assert_id(&list, 8, Some(0x0A));
    assert_id(&list, 9, Some(0x0B));
    assert_id(&list, 10, Some(0x00));
    assert_id(&list, 11, None);
    assert_id(&list, 12, Some(0x10));
    assert_id(&list, 13, Some(0x11));
    assert_id(&list, 14, Some(0x12));
    assert_id(&list, 15, Some(0x24));
    assert_id(&list, 16, Some(0x00));
    assert_eq!(out.manufacturer_bits(), 0);
}

#[test]
fn parse_established_asus_rog_pg27u() {
    let raw = base(ASUS);
    let out = EstablishedLegacy::new(&raw);
    let list = out.supported();

    assert_id(&list, 0, None);
    assert_id(&list, 1, None);
    assert_id(&list, 2, Some(0x04));
    assert_id(&list, 3, None);
    assert_id(&list, 4, None);
    assert_id(&list, 5, None);
    assert_id(&list, 6, None);
    assert_id(&list, 7, Some(0x09));
    assert_id(&list, 8, None);
    assert_id(&list, 9, None);
    assert_id(&list, 10, None);
    assert_id(&list, 11, None);
    assert_id(&list, 12, Some(0x10));
    assert_id(&list, 13, None);
    assert_id(&list, 14, None);
    assert_id(&list, 15, None);
    assert_id(&list, 16, None);
    assert_eq!(out.manufacturer_bits(), 0);
}

fn assert_id(list: &[Option<edid_info::base::dmt::Dmt>; 17], i: usize, id: Option<u8>) {
    assert_eq!(list[i].map(|d| d.id), id);
}
