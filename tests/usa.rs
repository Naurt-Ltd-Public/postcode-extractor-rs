use postcode_extractor::{PostalCodeRegexes, PostalCodes};

#[test]
fn test_usa() {
    let regex = PostalCodeRegexes::new();

    let test_address_str = "11600 NW 14th St, Pembroke Pines, FL 33026, USA";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::Unknown5Digit(us) => assert_eq!(us, "33026"),
        _ => panic!("Incorrect nation"),
    };
}

#[test]
fn test_usa_like_singapore() {
    let regex = PostalCodeRegexes::new();
    // USA Address that looks like Singapore
    let test_address_str = "239333 12th Ave New York 00125";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::Unknown5Digit(us) => assert_eq!(us, "00125"),
        _ => panic!("Incorrect nation"),
    };
}

#[test]
fn test_usa_like_uk() {
    let regex = PostalCodeRegexes::new();

    // USA disguised as UK

    let test_address_str = "E1 23RD St, Los Angeles, CA 90012";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::Unknown5Digit(us) => {
            assert_eq!(us, "90012");
        }
        _ => panic!("Incorrect nation"),
    }
}

#[test]
fn test_usa_plus_4() {
    let regex = PostalCodeRegexes::new();

    let test_address_str = "132-12 Jamaica Av., NY 00125-0123";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::US(us) => {
            assert_eq!(us.base, "00125");
            assert_eq!(us.additional.unwrap(), "0123")
        }
        _ => panic!("Incorrect nation"),
    }
}

#[test]
fn test_usa_like_nl() {
    let regex = PostalCodeRegexes::new();

    let test_address_str =
        "Estados Unidos Seu 20528, 6451 NW  102 nd Ave #7, Doral, Florida, 33178, United States";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();

    match reg {
        PostalCodes::Unknown5Digit(us) => {
            assert_eq!(us, "33178");
        }
        _ => panic!("Incorrect nation"),
    }
}

#[test]
fn test_usa_disguised_as_portugal() {
    let regex = PostalCodeRegexes::new();

    let test_address_str =
        "8465 W 44th Avenue Ste 119, PTY 2408-570 CF, Hialeah, Florida, 33018-2232, United States";

    let reg = regex
        .extract_postalcode_and_country(test_address_str, true)
        .unwrap();
    println!("{:?}", reg);
    match reg {
        PostalCodes::US(us) => {
            assert_eq!(us.base, "33018");
            assert_eq!(us.additional.unwrap(), "2232")
        }
        _ => panic!("Incorrect nation"),
    }
}
