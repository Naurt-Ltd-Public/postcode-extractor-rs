use crate::{PostCodeParser, countries::FiveDigitRegex};

#[test]
fn test_us_zipcode() {
    let us_regex = FiveDigitRegex::new();

    // Basic case
    let test_address_str = "132-12 Jamaica Av., NY 00125";

    let reg = us_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "00125");
    assert!(reg.additional.is_none());

    // ZIP+4

    let test_address_str = "132-12 Jamaica Av., NY 00125+0123";

    let reg = us_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "00125");
    assert_eq!(reg.additional.unwrap(), "0123");

    // ZIP-4

    let test_address_str = "132-12 Jamaica Av., NY 00125-0123";

    let reg = us_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "00125");
    assert_eq!(reg.additional.unwrap(), "0123");

    // Ensuring last Zip

    let test_address_str = "10012 5th St NY 00234-3214";

    let reg = us_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "00234");
    assert_eq!(reg.additional.unwrap(), "3214");

    // Another one

    let test_address_str = "11600 NW 14th St, Pembroke Pines, FL 33026, USA";

    let reg = us_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "33026");
    assert!(reg.additional.is_none());
}
