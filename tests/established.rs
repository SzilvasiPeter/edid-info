use edid_info::edid::established::Established;

const ACER: &[u8] = include_bytes!("data/acer_ek221q_h.edid");
const ASUS: &[u8] = include_bytes!("data/asus_rog_pg27u.edid");

#[test]
fn parse_established_acer_ek221q_h() {
    let raw: &[u8; 3] = ACER[35..38].try_into().expect("established bytes");
    let out = Established::parse(raw);

    assert!(out.t_720_400_70());
    assert!(!out.t_720_400_88());
    assert!(out.t_640_480_60());
    assert!(out.t_640_480_67());
    assert!(out.t_640_480_72());
    assert!(out.t_640_480_75());
    assert!(out.t_800_600_56());
    assert!(out.t_800_600_60());
    assert!(out.t_800_600_72());
    assert!(out.t_800_600_75());
    assert!(out.t_832_624_75());
    assert!(!out.t_1024_768_87i());
    assert!(out.t_1024_768_60());
    assert!(out.t_1024_768_70());
    assert!(out.t_1024_768_75());
    assert!(out.t_1280_1024_75());
    assert!(out.t_1152_870_75());
    assert_eq!(out.manufacturer_bits(), 0);
}

#[test]
fn parse_established_asus_rog_pg27u() {
    let raw: &[u8; 3] = ASUS[35..38].try_into().expect("established bytes");
    let out = Established::parse(raw);

    assert!(!out.t_720_400_70());
    assert!(!out.t_720_400_88());
    assert!(out.t_640_480_60());
    assert!(!out.t_640_480_67());
    assert!(!out.t_640_480_72());
    assert!(!out.t_640_480_75());
    assert!(!out.t_800_600_56());
    assert!(out.t_800_600_60());
    assert!(!out.t_800_600_72());
    assert!(!out.t_800_600_75());
    assert!(!out.t_832_624_75());
    assert!(!out.t_1024_768_87i());
    assert!(out.t_1024_768_60());
    assert!(!out.t_1024_768_70());
    assert!(!out.t_1024_768_75());
    assert!(!out.t_1280_1024_75());
    assert!(!out.t_1152_870_75());
    assert_eq!(out.manufacturer_bits(), 0);
}
