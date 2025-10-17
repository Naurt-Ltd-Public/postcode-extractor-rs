use crate::{PostCodeParser, countries::UkRegex};

#[test]
fn test_uk_postalcode() {
    let uk_regex = UkRegex::new();

    // Basic case
    let test_address_str = "12 Main Road, Shropshire, Wescestershire, CF34 0NP";

    let reg = uk_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "CF34 0NP");

    // Difficult case

    let test_address_str = "Some address here rood rm37pq";

    let reg = uk_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "RM3 7PQ");

    // Ensuring last postalcode

    let test_address_str = "CF34 0NP BN21 3LD";

    let reg = uk_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "BN21 3LD");
}
