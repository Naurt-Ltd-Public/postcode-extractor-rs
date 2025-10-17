use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_australia() {
    let regex = PostalCodeRegexes::new();

    let test_address_str = "6 Belbin Pl, Macquarie ACT 2614, Australia";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::Unknown4Digit(us) => assert_eq!(us, "2614"),
        _ => panic!("Incorrect nation"),
    };
}
