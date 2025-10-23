use postcode_extractor::{Country, evaluate_single_country};
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
fn test_individually() {
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
                let holder =
                    evaluate_single_country(&case.address, test_holder.country.clone(), true)
                        .expect(&message)
                        .expect(&message);

                assert_eq!(holder.base, case.postcode, "{}", message);

                if let Some(add) = &case.postcode_additional {
                    assert_eq!(add, holder.additional.as_ref().unwrap(), "{}", message);
                }
            }

            // Test not matching
            for case in &test_holder.tests.position_check.no_match {
                let message = format!("Evaluating {} in {:?}", case, path);

                let holder =
                    evaluate_single_country(&case, test_holder.country.clone(), true).unwrap();

                assert!(holder.is_none(), "{}", message);
            }

            // Test without length check
            // Test matching
            for case in &test_holder.tests.no_position_check.matches {
                let message = format!("Evaluating {} in {:?}", case.original, path);
                let holder =
                    evaluate_single_country(&case.original, test_holder.country.clone(), false)
                        .expect(&message)
                        .expect(&message);

                assert_eq!(holder.base, case.postcode, "{}", message);

                if let Some(add) = &case.postcode_additional {
                    assert_eq!(add, holder.additional.as_ref().unwrap(), "{}", message);
                }
            }

            // Test not matching
            for case in &test_holder.tests.no_position_check.no_match {
                let message = format!("Evaluating {} in {:?}", case, path);

                let holder =
                    evaluate_single_country(&case, test_holder.country.clone(), false).unwrap();

                assert!(holder.is_none(), "{}", message);
            }
        }
    }
}
