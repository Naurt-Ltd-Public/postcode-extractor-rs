use crate::{PostCodeParser, countries::CaRegex};

#[test]
fn test_canada() {
    let ca_regex = CaRegex::new();

    // Basic case
    let test_address_str = "123 Main St, Toronto, ON M5A 1A1, CAN";

    let reg = ca_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "M5A 1A1");

    // Making sure it doesn't get an Australian Postcode (also 4 digits but no LV-)

    let test_address_str = "6 Belbin Pl, Macquarie ACT 2614, Australia";

    let reg = ca_regex.evaluate(test_address_str, true);

    assert!(reg.is_none());
}
