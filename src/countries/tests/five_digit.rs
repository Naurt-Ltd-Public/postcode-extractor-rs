use crate::{PostCodeParser, countries::FiveDigitRegex};

#[test]
fn test_usa() {
    let five_regex = FiveDigitRegex::new();

    let test_address_str = "75 Farley Ave, Newark, NJ 07108-1234, USA";

    let reg = five_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "07108");
    assert_eq!(reg.additional.as_ref().unwrap(), "1234");
}

#[test]
fn test_peru() {
    // NOTE: We actually reuse US regex bc USA has 5 digits!
    let peru_regex = FiveDigitRegex::new();

    let test_address_str = "Country-Travel Tours, Avenida Tomás Valle, Callao 07036, Callao, Perú";

    let reg = peru_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "07036");
    assert!(reg.additional.is_none());
}
