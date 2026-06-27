use edid_info::base::established::EstTimings;

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_established_acer_ek221q_h() {
    let raw = core::array::from_fn(|i| ACER[i]);
    let out = EstTimings::new(&raw);
    let list: Vec<_> = out.iter().map(|d| d.id).collect();

    assert_eq!(
        list,
        [
            0x00, 0x04, 0x00, 0x05, 0x06, 0x08, 0x09, 0x0A, 0x0B, 0x00, 0x10, 0x11, 0x12, 0x24,
            0x00,
        ]
    );
    assert_eq!(out.manufacturer_bits(), 0);
}

#[test]
fn parse_established_asus_rog_pg27u() {
    let raw = core::array::from_fn(|i| ASUS[i]);
    let out = EstTimings::new(&raw);
    let list: Vec<_> = out.iter().map(|d| d.id).collect();

    assert_eq!(list, [0x04, 0x09, 0x10]);
    assert_eq!(out.manufacturer_bits(), 0);
}
