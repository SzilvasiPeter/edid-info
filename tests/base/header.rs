use edid_info::base::header::{DateInfo, Header};
use edid_info::common::{FailureKind, Version, WarningKind};

const ACER: &[u8] = include_bytes!("../data/ACER_EK221Q_H.edid");
const ASUS: &[u8] = include_bytes!("../data/ASUS_ROG_PG27U.edid");
const CM: &[u8] = include_bytes!("../data/CM__CM2400T.edid");
const CS: &[u8] = include_bytes!("../data/CS__1920x1080.edid");
const LPL: &[u8] = include_bytes!("../data/LPL_LP154W01.edid");
const MS: &[u8] = include_bytes!("../data/MS__HSD_1903-A00.edid");
const TK: &[u8] = include_bytes!("../data/TK@_tianma.edid");
const WG: &[u8] = include_bytes!("../data/WG@_UNKNOWN.edid");
const PHL_BAD_DATE: &[u8] = include_bytes!("../data/PHL_221V8.edid");
const TSB_MODEL_YEAR: &[u8] = include_bytes!("../data/TSB_TV.edid");

const HEADER_PATTERN: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

#[test]
fn parse_header_acer_ek221q_h() {
    let raw = core::array::from_fn(|i| ACER[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['A', 'C', 'R']);
    assert_eq!(out.product(), 2909);
    assert_eq!(out.serial(), 0x3480_002C);
    assert_eq!(
        out.date(),
        DateInfo::Manufacture {
            week: 48,
            year: 2023
        }
    );
    assert_eq!(out.version(), Version { major: 1, minor: 3 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: ACR, Product: 0B5D, Version: 1.3"
    );

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(
        validation.warnings,
        1 << (WarningKind::VersionIsDeprecated as u8),
    );
    assert_eq!(
        WarningKind::VersionIsDeprecated.message(),
        "Version is deprecated, use 1.4 or later"
    );
}

#[test]
fn parse_header_asus_rog_pg27u() {
    let raw = core::array::from_fn(|i| ASUS[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['A', 'U', 'S']);
    assert_eq!(out.product(), 10148);
    assert_eq!(out.serial(), 0x0001_b5bc);
    assert_eq!(
        out.date(),
        DateInfo::Manufacture {
            week: 30,
            year: 2018
        }
    );
    assert_eq!(out.version(), Version { major: 1, minor: 4 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: AUS, Product: 27A4, Version: 1.4"
    );

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(validation.warnings, 0);
}

#[test]
fn parse_header_cm_cm2400t() {
    let raw = core::array::from_fn(|i| CM[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['C', 'M', '_']);
    assert_eq!(out.product(), 9216);
    assert_eq!(out.serial(), 0x0101_0101);
    assert_eq!(
        out.date(),
        DateInfo::Manufacture {
            week: 45,
            year: 2017
        }
    );
    assert_eq!(out.version(), Version { major: 1, minor: 3 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: CM_, Product: 2400, Version: 1.3"
    );

    let validation = out.validate();
    assert!(!validation.is_valid());
    assert_eq!(
        validation.errors,
        1 << (FailureKind::ManufacturerIdIsInvalid as u8),
    );
    assert_eq!(
        FailureKind::ManufacturerIdIsInvalid.message(),
        "Manufacturer ID contains invalid bits"
    );
    assert_eq!(
        validation.warnings,
        1 << (WarningKind::VersionIsDeprecated as u8),
    );
    assert_eq!(
        WarningKind::VersionIsDeprecated.message(),
        "Version is deprecated, use 1.4 or later"
    );
}

#[test]
fn parse_header_cs_1920x1080() {
    let raw = core::array::from_fn(|i| CS[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['C', 'S', '_']);
    assert_eq!(out.product(), 21009);
    assert_eq!(out.serial(), 1025);
    assert_eq!(
        out.date(),
        DateInfo::Manufacture {
            week: 5,
            year: 2013
        }
    );
    assert_eq!(out.version(), Version { major: 1, minor: 4 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: CS_, Product: 5211, Version: 1.4"
    );

    let validation = out.validate();
    assert!(!validation.is_valid());
    assert_eq!(
        validation.errors,
        1 << (FailureKind::ManufacturerIdIsInvalid as u8),
    );
    assert_eq!(
        FailureKind::ManufacturerIdIsInvalid.message(),
        "Manufacturer ID contains invalid bits"
    );
    assert_eq!(validation.warnings, 0);
}

#[test]
fn parse_header_lpl_lp154w01_zeroweek() {
    let raw = core::array::from_fn(|i| LPL[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['L', 'P', 'L']);
    assert_eq!(out.product(), 51200);
    assert_eq!(out.serial(), 0);
    assert_eq!(
        out.date(),
        DateInfo::Manufacture {
            week: 0,
            year: 2006
        }
    );
    assert_eq!(out.version(), Version { major: 1, minor: 3 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: LPL, Product: C800, Version: 1.3"
    );

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(
        validation.warnings,
        (1 << (WarningKind::SerialNumberIsZero as u8))
            | (1 << (WarningKind::VersionIsDeprecated as u8)),
    );
    assert_eq!(
        WarningKind::SerialNumberIsZero.message(),
        "Serial number is zero"
    );
    assert_eq!(
        WarningKind::VersionIsDeprecated.message(),
        "Version is deprecated, use 1.4 or later"
    );
}

#[test]
fn parse_header_ms_hsd_1903_a00() {
    let raw = core::array::from_fn(|i| MS[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['M', 'S', '_']);
    assert_eq!(out.product(), 60);
    assert_eq!(out.serial(), 0);
    assert_eq!(
        out.date(),
        DateInfo::Manufacture {
            week: 20,
            year: 2021
        }
    );
    assert_eq!(out.version(), Version { major: 1, minor: 2 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: MS_, Product: 003C, Version: 1.2"
    );

    let validation = out.validate();
    assert!(!validation.is_valid());
    assert_eq!(
        validation.errors,
        1 << (FailureKind::ManufacturerIdIsInvalid as u8),
    );
    assert_eq!(
        FailureKind::ManufacturerIdIsInvalid.message(),
        "Manufacturer ID contains invalid bits"
    );

    assert_eq!(
        validation.warnings,
        (1 << (WarningKind::SerialNumberIsZero as u8))
            | (1 << (WarningKind::VersionIsDeprecated as u8)),
    );
    assert_eq!(
        WarningKind::SerialNumberIsZero.message(),
        "Serial number is zero"
    );
    assert_eq!(
        WarningKind::VersionIsDeprecated.message(),
        "Version is deprecated, use 1.4 or later"
    );
}

#[test]
fn parse_header_tk_tianma() {
    let raw = core::array::from_fn(|i| TK[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['T', 'K', '@']);
    assert_eq!(out.product(), 8427);
    assert_eq!(out.serial(), 0);
    assert_eq!(
        out.date(),
        DateInfo::Manufacture {
            week: 31,
            year: 2018
        }
    );
    assert_eq!(out.version(), Version { major: 1, minor: 4 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: TK@, Product: 20EB, Version: 1.4"
    );

    let validation = out.validate();
    assert!(!validation.is_valid());
    assert_eq!(
        validation.errors,
        1 << (FailureKind::ManufacturerIdIsInvalid as u8),
    );
    assert_eq!(
        FailureKind::ManufacturerIdIsInvalid.message(),
        "Manufacturer ID contains invalid bits"
    );
    assert_eq!(
        validation.warnings,
        1 << (WarningKind::SerialNumberIsZero as u8),
    );
    assert_eq!(
        WarningKind::SerialNumberIsZero.message(),
        "Serial number is zero"
    );
}

#[test]
fn parse_header_wg_unknown() {
    let raw = core::array::from_fn(|i| WG[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['W', 'G', '@']);
    assert_eq!(out.product(), 0);
    assert_eq!(out.serial(), 0);
    assert_eq!(
        out.date(),
        DateInfo::Manufacture {
            week: 0,
            year: 2007
        }
    );
    assert_eq!(out.version(), Version { major: 1, minor: 1 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: WG@, Product: 0000, Version: 1.1"
    );

    let validation = out.validate();
    assert!(!validation.is_valid());
    assert_eq!(
        validation.errors,
        1 << (FailureKind::ManufacturerIdIsInvalid as u8),
    );
    assert_eq!(
        FailureKind::ManufacturerIdIsInvalid.message(),
        "Manufacturer ID contains invalid bits"
    );
    assert_eq!(
        validation.warnings,
        (1 << (WarningKind::ProductCodeIsZero as u8))
            | (1 << (WarningKind::SerialNumberIsZero as u8))
            | (1 << (WarningKind::VersionIsDeprecated as u8)),
    );
    assert_eq!(
        WarningKind::ProductCodeIsZero.message(),
        "Product code is zero"
    );
    assert_eq!(
        WarningKind::SerialNumberIsZero.message(),
        "Serial number is zero"
    );
    assert_eq!(
        WarningKind::VersionIsDeprecated.message(),
        "Version is deprecated, use 1.4 or later"
    );
}

#[test]
fn parse_header_phl_bad_date() {
    let raw = core::array::from_fn(|i| PHL_BAD_DATE[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['P', 'H', 'L']);
    assert_eq!(out.product(), 49681);
    assert_eq!(out.serial(), 0x0000_0e7b);
    assert_eq!(
        out.date(),
        DateInfo::Manufacture {
            week: 70,
            year: 2034
        }
    );
    assert_eq!(out.version(), Version { major: 1, minor: 3 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: PHL, Product: C211, Version: 1.3"
    );

    let validation = out.validate();
    assert!(!validation.is_valid());
    assert_eq!(
        validation.errors,
        1 << (FailureKind::ManufactureWeekIsInvalid as u8),
    );
    assert_eq!(
        FailureKind::ManufactureWeekIsInvalid.message(),
        "Manufacture week value is invalid"
    );
    assert_eq!(
        validation.warnings,
        1 << (WarningKind::VersionIsDeprecated as u8),
    );
    assert_eq!(
        WarningKind::VersionIsDeprecated.message(),
        "Version is deprecated, use 1.4 or later"
    );
}

#[test]
fn parse_header_tsb_model_year() {
    let raw = core::array::from_fn(|i| TSB_MODEL_YEAR[i]);
    let out = Header::new(&raw);
    assert_eq!(out.pattern(), HEADER_PATTERN);
    assert_eq!(out.manufacturer(), ['T', 'S', 'B']);
    assert_eq!(out.product(), 272);
    assert_eq!(out.serial(), 0x0101_0101);
    assert_eq!(out.date(), DateInfo::ModelYear { year: 2013 });
    assert_eq!(out.version(), Version { major: 1, minor: 3 });
    assert_eq!(
        out.to_string(),
        "Manufacturer: TSB, Product: 0110, Version: 1.3"
    );

    let validation = out.validate();
    assert!(validation.is_valid());
    assert_eq!(validation.errors, 0);
    assert_eq!(
        validation.warnings,
        1 << (WarningKind::VersionIsDeprecated as u8),
    );
    assert_eq!(
        WarningKind::VersionIsDeprecated.message(),
        "Version is deprecated, use 1.4 or later"
    );
}
