use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_japan() {
    let jp_regex: PostalCodeRegexes = PostalCodeRegexes::new();

    // Basic case
    let test_address_str = "123-4567東京都新宿区新宿二丁目1-9日本";

    let reg = jp_regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::JP(s) => assert_eq!(s, "123-4567"),
        _ => panic!("Wrong country!"),
    }
}

#[test]
fn test_japan_postal_symbol() {
    let jp_regex = PostalCodeRegexes::new();
    let symbols = &["〒", "〠", "⮗", "〶"];
    let test_address_str = "123-4567東京都新宿区新宿二丁目1-9日本";
    for symbol in symbols {
        let haystack = format!("{}{}", symbol, test_address_str);
        let reg = jp_regex
            .extract_postalcode_and_country(&haystack, true)
            .unwrap();

        match reg {
            PostalCodes::JP(s) => assert_eq!(s, "123-4567"),
            _ => panic!("Wrong country!"),
        }
    }
}
