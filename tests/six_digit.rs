use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_singapore() {
    let regex = PostalCodeRegexes::new();

    let test_address_str = "23 Kim Yam Rd, Singapore 239333";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::Unknown6Digit(sg) => assert_eq!(sg, "239333"),
        _ => panic!("Incorrect nation"),
    };
}
