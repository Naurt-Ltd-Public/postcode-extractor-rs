use crate::{PostCodeParser, countries::SixRegex};

#[test]
fn test_sg_postalcode() {
    let six_regex = SixRegex::new();

    // Basic case
    let test_address_str = "58 Oxley Rd, Singapore 238643";

    let reg = six_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "238643");

    // Difficult case

    let test_address_str = "23 Kim Yam Rd, Singapore 239333";

    let reg = six_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "239333");

    // Ensuring last postalcode

    let test_address_str = "239333 238643";

    let reg = six_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "238643");

    // Ensuring last postalcode

    let test_address_str = "parastakiz ke pass, Bada Bazaar, Sagar, Madhya Pradesh 470002, India";

    let reg = six_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "470002");
}
