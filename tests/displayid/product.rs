use edid_info::edid::displayid::product::ProductIdent;

#[test]
fn test_product_ident_parse() {
    let data = [
        0x01, 0x02, 0x03, // OUI
        0x04, 0x05, // Product
        0x06, 0x07, 0x08, 0x09, // Serial
        0x0A, // Week
        0x0B, // Year (2000 + 11 = 2011)
        0x04, // Name length
        b'T', b'E', b'S', b'T', // Name
    ];

    let ident = ProductIdent::parse(&data).expect("should parse");
    assert_eq!(ident.oui(), [0x01, 0x02, 0x03]);
    assert_eq!(ident.product(), 0x0504);
    assert_eq!(ident.serial(), Some(0x0908_0706));
    assert_eq!(ident.week(), 0x0A);
    assert_eq!(ident.year(), 2011);
    assert_eq!(ident.name(), Some("TEST"));
}

#[test]
fn test_product_ident_parse_short() {
    let data = [0; 11];
    assert!(ProductIdent::parse(&data).is_none());
}

#[test]
fn test_product_ident_parse_no_name() {
    let data = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x00,
    ];
    let ident = ProductIdent::parse(&data).expect("should parse");
    assert_eq!(ident.name(), None);
}
