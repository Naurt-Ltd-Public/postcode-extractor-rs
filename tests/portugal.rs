use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_portugal() {
    let regex = PostalCodeRegexes::new();

    let input = "R 3 Br Da Junqueira 13, 4405-872 Vila Nova De Gaia, Portugal";

    let post = regex.extract_postalcode_and_country(input, true).unwrap();

    match post {
        PostalCodes::PT(br) => assert_eq!(br, "4405-872"),
        _ => panic!("Wrong country detected"),
    }
}
