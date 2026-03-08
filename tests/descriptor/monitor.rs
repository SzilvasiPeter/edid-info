use edid_info::edid::descriptor::monitor::{DescTag, MonitorDesc};

const ACER: &[u8] = include_bytes!("../data/acer_ek221q_h.edid");
const ASUS: &[u8] = include_bytes!("../data/asus_rog_pg27u.edid");

#[test]
fn parse_serial_descriptor_acer_ek221q_h() {
    let serial_raw: &[u8; 18] = ACER[72..90].try_into().expect("serial descriptor bytes");

    let serial = MonitorDesc::parse(serial_raw).expect("serial descriptor parse");
    assert_eq!(serial.tag(), DescTag::SerialNumber);
    assert_eq!(serial.serial(), Some("13480002C3W01"));
    assert_eq!(serial.name(), None);
    assert_eq!(serial.text(), None);
}

#[test]
fn parse_serial_descriptor_asus_rog_pg27u() {
    let serial_raw: &[u8; 18] = ASUS[72..90].try_into().expect("serial descriptor bytes");

    let serial = MonitorDesc::parse(serial_raw).expect("serial descriptor parse");
    assert_eq!(serial.tag(), DescTag::SerialNumber);
    assert_eq!(serial.serial(), Some("#ASM5Wbbmo37d"));
    assert_eq!(serial.name(), None);
    assert_eq!(serial.text(), None);
}

#[test]
fn parse_product_name_descriptor_acer_ek221q_h() {
    let name_raw: &[u8; 18] = ACER[90..108].try_into().expect("name descriptor bytes");
    let name = MonitorDesc::parse(name_raw).expect("name descriptor parse");
    assert_eq!(name.tag(), DescTag::MonitorName);
    assert_eq!(name.name(), Some("EK221Q H"));
    assert_eq!(name.serial(), None);
    assert_eq!(name.text(), None);
}

#[test]
fn parse_product_name_descriptor_asus_rog_pg27u() {
    let name_raw: &[u8; 18] = ASUS[108..126].try_into().expect("name descriptor bytes");
    let name = MonitorDesc::parse(name_raw).expect("name descriptor parse");
    assert_eq!(name.tag(), DescTag::MonitorName);
    assert_eq!(name.name(), Some("ROG PG27U"));
    assert_eq!(name.serial(), None);
    assert_eq!(name.text(), None);
}
