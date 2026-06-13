use edid_info::base::established::EstTimings;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

fn base(raw: &[u8]) -> [u8; 128] {
    core::array::from_fn(|i| raw[i])
}

#[test]
fn parse_established_acer_ek221q_h() {
    let raw = base(ACER);
    let out = EstTimings::new(&raw);
    let list: Vec<_> = out.iter().map(|d| d.id).collect();

    assert_ids(
        &list,
        &[
            0x00, 0x04, 0x00, 0x05, 0x06, 0x08, 0x09, 0x0A, 0x0B, 0x00, 0x10, 0x11, 0x12, 0x24,
            0x00,
        ],
    );
    assert_eq!(out.manufacturer_bits(), 0);
}

#[test]
fn parse_established_asus_rog_pg27u() {
    let raw = base(ASUS);
    let out = EstTimings::new(&raw);
    let list: Vec<_> = out.iter().map(|d| d.id).collect();

    assert_ids(&list, &[0x04, 0x09, 0x10]);
    assert_eq!(out.manufacturer_bits(), 0);
}

fn assert_ids(list: &[u8], ids: &[u8]) {
    assert_eq!(list, ids);
}
