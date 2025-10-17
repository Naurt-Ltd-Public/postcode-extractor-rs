use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_latvia() {
    let regex = PostalCodeRegexes::new();

    let test_address_str = "Pērses iela, Centra rajons, Rīga, LV-1011, Latvia";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::Lv(lv) => assert_eq!(lv, "LV-1011"),
        _ => panic!("Incorrect nation"),
    };
}
