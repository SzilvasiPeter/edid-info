use edid_info::base::descriptor::monitor::{DisplayDescriptor, Monitor};
use edid_info::base::header::Header;
use edid_info::common::{BLOCK_LEN, Version};

fn is_legacy(raw_base: &[u8; BLOCK_LEN]) -> bool {
    Header::new(raw_base).version() < Version { major: 1, minor: 3 }
}

const ACER: &[u8] = include_bytes!("../../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../../data/ASUS_ROG_PG27U.edid");
const LENOVO: &[u8] = include_bytes!("../../data/LENOVO_LP156WF9_SPK2.edid");
const ROL: &[u8] = include_bytes!("../../data/ROL_ROLSEN_C707N.edid");
const VIT: &[u8] = include_bytes!("../../data/VIT_VT988.edid");

#[test]
fn parse_serial_descriptor_acer_ek221q_h() {
    let serial_raw: [u8; 18] = std::array::from_fn(|i| ACER[72 + i]);
    let base: [u8; BLOCK_LEN] = ACER[0..BLOCK_LEN].try_into().unwrap();
    let serial = Monitor::parse(&serial_raw, is_legacy(&base));
    let desc = serial.descriptor();
    assert!(matches!(desc, DisplayDescriptor::SerialNumber(_)));
    if let DisplayDescriptor::SerialNumber(sn) = desc {
        assert_eq!(sn.text(), "13480002C3W01");
    }
}

#[test]
fn parse_serial_descriptor_asus_rog_pg27u() {
    let serial_raw: [u8; 18] = std::array::from_fn(|i| ASUS[72 + i]);
    let base: [u8; BLOCK_LEN] = ASUS[0..BLOCK_LEN].try_into().unwrap();
    let serial = Monitor::parse(&serial_raw, is_legacy(&base));
    let desc = serial.descriptor();
    assert!(matches!(desc, DisplayDescriptor::SerialNumber(_)));
    if let DisplayDescriptor::SerialNumber(sn) = desc {
        assert_eq!(sn.text(), "#ASM5Wbbmo37d");
    }
}

#[test]
fn parse_product_name_descriptor_acer_ek221q_h() {
    let name_raw: [u8; 18] = std::array::from_fn(|i| ACER[90 + i]);
    let base: [u8; BLOCK_LEN] = ACER[0..BLOCK_LEN].try_into().unwrap();
    let name = Monitor::parse(&name_raw, is_legacy(&base));
    let desc = name.descriptor();
    assert!(matches!(desc, DisplayDescriptor::MonitorName(_)));
    if let DisplayDescriptor::MonitorName(sn) = desc {
        assert_eq!(sn.text(), "EK221Q H");
    }
}

#[test]
fn parse_product_name_descriptor_asus_rog_pg27u() {
    let name_raw: [u8; 18] = std::array::from_fn(|i| ASUS[108 + i]);
    let base: [u8; BLOCK_LEN] = ASUS[0..BLOCK_LEN].try_into().unwrap();
    let name = Monitor::parse(&name_raw, is_legacy(&base));
    let desc = name.descriptor();
    assert!(matches!(desc, DisplayDescriptor::MonitorName(_)));
    if let DisplayDescriptor::MonitorName(sn) = desc {
        assert_eq!(sn.text(), "ROG PG27U");
    }
}

#[test]
fn parse_text_descriptor_lenovo_lp156wf9_spk2() {
    let text_raw: [u8; 18] = std::array::from_fn(|i| LENOVO[108 + i]);
    let base: [u8; BLOCK_LEN] = LENOVO[0..BLOCK_LEN].try_into().unwrap();
    let text = Monitor::parse(&text_raw, is_legacy(&base));
    let desc = text.descriptor();
    assert!(matches!(desc, DisplayDescriptor::Text(_)));
    if let DisplayDescriptor::Text(sn) = desc {
        assert_eq!(sn.text(), "LP156WF9-SPK2");
    }
}

#[test]
fn parse_product_name_descriptor_rol_rolsen_c707n() {
    let name_raw: [u8; 18] = std::array::from_fn(|i| ROL[90 + i]);
    let base: [u8; BLOCK_LEN] = ROL[0..BLOCK_LEN].try_into().unwrap();
    let name = Monitor::parse(&name_raw, is_legacy(&base));
    let desc = name.descriptor();
    assert!(matches!(desc, DisplayDescriptor::MonitorName(_)));
    if let DisplayDescriptor::MonitorName(sn) = desc {
        assert_eq!(sn.text(), "ROLSEN C707N");
    }
}

#[test]
fn parse_std_timings2_descriptor_rol_rolsen_c707n() {
    let std_raw: [u8; 18] = std::array::from_fn(|i| ROL[72 + i]);
    let base: [u8; BLOCK_LEN] = ROL[0..BLOCK_LEN].try_into().unwrap();
    let std = Monitor::parse(&std_raw, is_legacy(&base));
    let desc = std.descriptor();
    assert!(matches!(desc, DisplayDescriptor::StdTimings2(_)));
}

#[test]
fn parse_dcm_descriptor_vit_vt988() {
    let dcm_raw: [u8; 18] = std::array::from_fn(|i| VIT[108 + i]);
    let base: [u8; BLOCK_LEN] = VIT[0..BLOCK_LEN].try_into().unwrap();
    let dcm = Monitor::parse(&dcm_raw, is_legacy(&base));
    let desc = dcm.descriptor();
    assert!(matches!(desc, DisplayDescriptor::Dcm(_)));
}
