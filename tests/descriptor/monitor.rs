use edid_info::edid::descriptor::monitor::{DescTag, MonitorDesc};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");
const LENOVO: &[u8] = include_bytes!("../data/LENOVO_LP156WF9_SPK2.edid");
const ROL: &[u8] = include_bytes!("../data/ROL_ROLSEN_C707N.edid");
const VIT: &[u8] = include_bytes!("../data/VIT_VT988.edid");

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

#[test]
fn parse_text_descriptor_lenovo_lp156wf9_spk2() {
    let text_raw: &[u8; 18] = LENOVO[108..126].try_into().expect("text descriptor bytes");
    let text = MonitorDesc::parse(text_raw).expect("text descriptor parse");
    assert_eq!(text.tag(), DescTag::Text);
    assert_eq!(text.text(), Some("LP156WF9-SPK2"));
    assert_eq!(text.name(), None);
    assert_eq!(text.serial(), None);
}

#[test]
fn parse_product_name_descriptor_rol_rolsen_c707n() {
    let name_raw: &[u8; 18] = ROL[90..108].try_into().expect("name descriptor bytes");
    let name = MonitorDesc::parse(name_raw).expect("name descriptor parse");
    assert_eq!(name.tag(), DescTag::MonitorName);
    assert_eq!(name.name(), Some("ROLSEN C707N"));
    assert_eq!(name.serial(), None);
    assert_eq!(name.text(), None);
}

#[test]
fn parse_std_timings2_descriptor_rol_rolsen_c707n() {
    let std_raw: &[u8; 18] = ROL[72..90].try_into().expect("std timings2 bytes");
    let std = MonitorDesc::parse(std_raw).expect("std timings2 parse");
    assert_eq!(std.tag(), DescTag::StdTimings2);
    assert!(std.std2().is_some());
}

#[test]
fn parse_dcm_descriptor_vit_vt988() {
    let dcm_raw: &[u8; 18] = VIT[108..126].try_into().expect("dcm descriptor bytes");
    let dcm = MonitorDesc::parse(dcm_raw).expect("dcm descriptor parse");
    assert_eq!(dcm.tag(), DescTag::Dcm);
}
