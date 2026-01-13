use postcode_extractor::{
    Country, FIVE_DIGIT_ADDITIONAL_NATIONS, FIVE_DIGIT_NATIONS, FIVE_DIGIT_WITH_SPACE_NATIONS,
    FOUR_DIGIT_NATIONS, PostcodeWrapper, SIX_DIGIT_NATIONS, evaluate_all_countries,
};
use serde::Deserialize;
use std::fs;
use walkdir::WalkDir;

#[derive(Deserialize)]
pub struct TestHolder {
    pub country: Country,
    pub tests: Tests,
}

#[derive(Deserialize)]
pub struct Tests {
    pub position_check: PositionTestCases,
    pub no_position_check: NoPositionTestCases,
}

#[derive(Deserialize)]
pub struct PositionTestCases {
    pub matches: Vec<PositionMatchTest>,
    pub no_match: Vec<String>,
}

#[derive(Deserialize)]
pub struct PositionMatchTest {
    pub address: String,
    pub postcode: String,
    pub postcode_additional: Option<String>,
}

#[derive(Deserialize)]
pub struct NoPositionTestCases {
    pub matches: Vec<NoPositionMatchTest>,
    pub no_match: Vec<String>,
}

#[derive(Deserialize)]
pub struct NoPositionMatchTest {
    pub original: String,
    pub postcode: String,
    pub postcode_additional: Option<String>,
}

#[test]
fn test_all() {
    for doc in WalkDir::new("postalcode-extractor/tests") {
        let entry = doc.unwrap();

        let path = entry.into_path();
        if path.is_file() {
            let text = fs::read_to_string(path.clone()).unwrap();

            let test_holder: TestHolder = serde_json::from_str(&text).unwrap();

            // Test with length check
            // Test goood matches
            for case in &test_holder.tests.position_check.matches {
                let message = format!("Evaluating {} in {:?}", case.address, path);
                let wrapper = evaluate_all_countries(&case.address, true)
                    .expect(&message)
                    .expect(&message);

                verify_country(&wrapper, &test_holder, &message);

                let holder = wrapper.postcode;

                assert_eq!(holder.base, case.postcode, "{}", message);

                if let Some(add) = &case.postcode_additional {
                    assert_eq!(add, holder.additional.as_ref().unwrap(), "{}", message);
                }
            }

            // Test without length check
            // Test matching
            for case in &test_holder.tests.no_position_check.matches {
                let message = format!("Evaluating {} in {:?}", case.original, path);
                let wrapper = evaluate_all_countries(&case.original, false)
                    .expect(&message)
                    .expect(&message);

                verify_country(&wrapper, &test_holder, &message);

                let holder = wrapper.postcode;
                assert_eq!(holder.base, case.postcode, "{}", message);

                if let Some(add) = &case.postcode_additional {
                    assert_eq!(add, holder.additional.as_ref().unwrap(), "{}", message);
                }
            }
        }
    }
}

fn verify_country(wrapper: &PostcodeWrapper, test_holder: &TestHolder, debug_msg: &str) {
    if SIX_DIGIT_NATIONS.contains(&test_holder.country) {
        if wrapper.country != Country::Unknown6Digit
            && !(wrapper.country == Country::TW && test_holder.country == Country::TW)
        {
            panic!(
                "Expected {:?}; got {:?} {}",
                test_holder.country, wrapper.country, debug_msg
            )
        }
    } else if FIVE_DIGIT_ADDITIONAL_NATIONS.contains(&test_holder.country) {
        if wrapper.country != Country::Unknown5DigitAdditional
            && wrapper.country != Country::Unknown5Digit
        {
            panic!(
                "Expected {:?}; got {:?} {}",
                test_holder.country, wrapper.country, debug_msg
            )
        }
    } else if FIVE_DIGIT_NATIONS.contains(&test_holder.country) {
        if wrapper.country != Country::Unknown5Digit
            && wrapper.country != Country::Unknown5DigitAdditional
        {
            panic!(
                "Expected {:?}; got {:?} {}",
                test_holder.country, wrapper.country, debug_msg
            )
        }
    } else if FOUR_DIGIT_NATIONS.contains(&test_holder.country) {
        if wrapper.country != Country::Unknown4Digit
            && !(wrapper.country == Country::CY && test_holder.country == Country::CY)
            && !(wrapper.country == Country::LU && test_holder.country == Country::LU)
        {
            panic!(
                "Expected {:?}; got {:?} {}",
                test_holder.country, wrapper.country, debug_msg
            )
        }
    } else if FIVE_DIGIT_WITH_SPACE_NATIONS.contains(&test_holder.country) {
        if wrapper.country != Country::Unknown5DigitSpace
            && !(wrapper.country == Country::CZ && test_holder.country == Country::CZ)
            && !(wrapper.country == Country::GR && test_holder.country == Country::GR)
        {
            panic!(
                "Expected {:?}; got {:?} {}",
                test_holder.country, wrapper.country, debug_msg
            )
        }
    } else {
        assert_eq!(wrapper.country, test_holder.country, "{}", debug_msg);
    }
}
