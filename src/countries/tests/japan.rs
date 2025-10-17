use crate::{PostCodeParser, countries::JpRegex};

#[test]
fn test_japan() {
    let jp_regex = JpRegex::new();

    // Basic case
    let test_address_str = "123-4567東京都新宿区新宿二丁目1-9日本";

    let reg = jp_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "123-4567");
}

#[test]
fn test_japan_postal_symbol() {
    let jp_regex = JpRegex::new();
    let symbols = &["〒", "〠", "⮗", "〶"];
    let test_address_str = "123-4567東京都新宿区新宿二丁目1-9日本";
    for symbol in symbols {
        let haystack = format!("{}{}", symbol, test_address_str);
        let reg = jp_regex.evaluate(&haystack, true).unwrap();

        assert_eq!(reg.base, "123-4567");
    }
}
