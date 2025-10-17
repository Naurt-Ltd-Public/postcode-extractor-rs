use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_liverpool() {
    let regex = PostalCodeRegexes::new();

    let input = "Baltimore Buildings, 17 Rodney St, Liverpool L1 9EF";

    let post = regex.extract_postalcode_and_country(input, true).unwrap();

    match post {
        PostalCodes::UK(uk) => assert_eq!(uk, "L1 9EF"),
        _ => panic!("Wrong country detected"),
    }
}

#[test]
fn test_difficult_uk() {
    let regex = PostalCodeRegexes::new();
    // UK disguised as USA

    // This can actually happen this is how A*eripoors do international shipping
    let test_address_str = "123 Oxford St, London, W1D 4NR, 10001";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::UK(uk) => assert_eq!(uk, "W1D 4NR"),
        _ => panic!("Incorrect nation"),
    };
}

#[test]
fn test_uk() {
    let regex = PostalCodeRegexes::new();

    // UK - Basic
    let test_address_str = "12 Main Road, Shropshire, Wescestershire, CF34 0NP";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::UK(uk) => assert_eq!(uk, "CF34 0NP"),
        _ => panic!("Incorrect nation"),
    };
}
