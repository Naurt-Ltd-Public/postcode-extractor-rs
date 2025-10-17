use crate::{PostCodeParser, countries::NlRegex};

#[test]
fn test_nl_postalcode() {
    let nl_regex = NlRegex::new();

    // Basic case
    let test_address_str = "Regulierenstraat 10, 3232 BR Brielle, Netherlands";

    let reg = nl_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "3232 BR");
}
