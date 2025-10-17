use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_brazil() {
    let regex = PostalCodeRegexes::new();

    let input =
        "Fulano da Silva, Rua Governador Roberto Silveira, 108, Centro, Macaé - RJ, 27910-050";

    let post = regex.extract_postalcode_and_country(input, true).unwrap();

    match post {
        PostalCodes::BR(br) => assert_eq!(br, "27910-050"),
        _ => panic!("Wrong country detected"),
    }
}
