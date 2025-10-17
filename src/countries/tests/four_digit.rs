use crate::{PostCodeParser, countries::FourRegex};

#[test]
fn test_australia() {
    let four_postal = FourRegex::new();
    let test_address_str = "6 Belbin Pl, Macquarie ACT 2614, Australia";

    let reg = four_postal.evaluate(test_address_str, true).unwrap();

    assert_eq!(reg.base, "2614");
}
