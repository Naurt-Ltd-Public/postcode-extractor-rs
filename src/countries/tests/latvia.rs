use crate::{PostCodeParser, countries::LvRegex};

#[test]
fn test_latvia() {
    let lv_regex = LvRegex::new();

    // Basic case
    let test_address_str = "Pērses iela, Centra rajons, Rīga, LV-1011, Latvia";

    let reg = lv_regex.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "LV-1011");

    // Making sure it doesn't get an Australian Postcode (also 4 digits but no LV-)

    let test_address_str = "6 Belbin Pl, Macquarie ACT 2614, Australia";

    let reg = lv_regex.evaluate(test_address_str, true);

    assert!(reg.is_none());
}
