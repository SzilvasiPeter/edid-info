use edid_info::base::descriptor::monitor::Monitor;

const ACER: &[u8] = include_bytes!("../../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../../data/ASUS_ROG_PG27U.edid");

#[test]
fn parse_std3_not_present_acer_ek221q_h() {
    let raw: [u8; 18] = std::array::from_fn(|i| ACER[90 + i]);
    let std3 = Monitor::parse(&raw).and_then(|desc| desc.std3());
    assert!(std3.is_none());
}

#[test]
fn parse_std3_not_present_asus_rog_pg27u() {
    let raw: [u8; 18] = std::array::from_fn(|i| ASUS[90 + i]);
    let std3 = Monitor::parse(&raw).and_then(|desc| desc.std3());
    assert!(std3.is_none());
}
