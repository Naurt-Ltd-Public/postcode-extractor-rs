use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_estonia() {
    let regex = PostalCodeRegexes::new();

    let test_address_str = "Valuste põik 2, Lihula, 90303 Pärnu maakond, Estonia";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::Unknown5Digit(us) => assert_eq!(us, "90303"),
        _ => panic!("Incorrect nation"),
    };
}

#[test]
fn test_peru() {
    let regex = PostalCodeRegexes::new();
    let test_address_str = "Country-Travel Tours, Avenida Tomás Valle, Callao 07036, Callao, Perú";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::Unknown5Digit(us) => assert_eq!(us, "07036"),
        _ => panic!("Incorrect nation"),
    };
}
