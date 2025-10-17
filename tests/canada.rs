use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_ca() {
    let regex = PostalCodeRegexes::new();

    // UK - Basic
    let test_address_str = "407 Victor St, Winnipeg, MB R3G 1P8, Canada";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::CA(ca) => assert_eq!(ca, "R3G 1P8"),
        _ => panic!("Incorrect nation"),
    };
}
