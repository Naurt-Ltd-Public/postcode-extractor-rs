use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_nl() {
    let regex = PostalCodeRegexes::new();

    let test_address_str = "Hoogstraat 299, 5654 NB Eindhoven, Netherlands";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::NL(nl) => assert_eq!(nl, "5654 NB"),
        _ => panic!("Incorrect nation"),
    };
}
