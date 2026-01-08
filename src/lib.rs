use std::collections::BTreeMap;

use include_dir::{Dir, include_dir};
use once_cell::sync::Lazy;
use regex::{Match, Regex};

mod country;
mod json_models;

use crate::json_models::{PositionLogic, RegexJson};

pub use country::{
    Country, FIVE_DIGIT_ADDITIONAL_NATIONS, FIVE_DIGIT_NATIONS, FIVE_DIGIT_WITH_SPACE_NATIONS,
    FOUR_DIGIT_NATIONS, SIX_DIGIT_NATIONS, UNIQUE_COUNTRIES,
};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/postalcode-extractor/regex");

struct RegexWrapper {
    pub regex: Regex,
    pub position_logic: PositionLogic,
}

static ALL: Lazy<BTreeMap<Country, RegexWrapper>> = Lazy::new(|| {
    let mut m = BTreeMap::new();
    for file in TEMPLATES_DIR.files() {
        let text = file.contents_utf8().expect("Must be UTF8");
        let regex_json: RegexJson = serde_json::from_str(text).unwrap();

        assert!(regex_json.regex.position_logic.position >= 0.0);
        assert!(regex_json.regex.position_logic.position <= 1.0);

        let regex = Regex::new(&regex_json.regex.engines.rust).unwrap();

        let regex_wrapper = RegexWrapper {
            regex: regex,
            position_logic: regex_json.regex.position_logic.clone(),
        };

        m.insert(regex_json.country.clone(), regex_wrapper);
    }
    m
});

#[derive(Debug, Clone)]
pub struct PostcodeHolder {
    pub base: String,
    pub additional: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PostcodeWrapper {
    pub country: Country,
    pub postcode: PostcodeHolder,
}

#[derive(Debug)]
pub enum PostcodeError {
    UnsupportedCountry,
}

/// Used to parse a postcode from an address of an already known country
///
/// Use `check_position` to control whether you are evaluating a whole address
/// or a single postcode. For example
///
/// - `check_position = true`: "15 Main Road, EN35 0RS" will find "EN35 0RS"
/// - `check_position = false`: "EN35 0RS" will find "EN35 0RS"
///  
///
pub fn evaluate_single_country(
    haystack: &str,
    country: Country,
    check_position: bool,
) -> Result<Option<PostcodeHolder>, PostcodeError> {
    let regex = ALL.get(&country).ok_or(PostcodeError::UnsupportedCountry)?;

    let postalcode_captures = regex.regex.captures_iter(haystack);

    // TODO: Sometimes we will need to take the first (e.g. Korea, Japan).
    // The position logic struct can probably take care of that
    let captures = match postalcode_captures.last() {
        Some(x) => x,
        None => return Ok(None),
    };

    let best_match = match captures.name("postcode") {
        Some(x) => x,
        None => return Ok(None),
    };

    if check_position {
        if !check_positions(haystack, &best_match, &regex.position_logic) {
            return Ok(None);
        }
    }

    let additional = if FIVE_DIGIT_ADDITIONAL_NATIONS.contains(&country) {
        if let Some(add_match) = captures.name("postcode_additional") {
            Some(add_match.as_str().to_string())
        } else {
            None
        }
    } else {
        None
    };

    let best_match = best_match.as_str().to_string();
    Ok(Some(PostcodeHolder {
        base: best_match,
        additional: additional,
    }))
}

fn check_positions(haystack: &str, mat: &Match, position_logic: &PositionLogic) -> bool {
    let hay_len = haystack.chars().count() as f64;
    let match_char_idex = haystack[..mat.start()].chars().count() as f64;

    let func = position_logic.operation.as_function();

    return func(match_char_idex / hay_len, position_logic.position);
}

/// Will attempt to parse a postcode from an address or a single postcode, and
/// determine which country it comes from.
///
/// Use `check_position` to control whether you are evaluating a whole address
/// or a single postcode. For example
///
/// - `check_position = true`: "15 Main Road, EN35 0RS" will find "EN35 0RS" and identify the country as United Kingdom
/// - `check_position = false`: "EN35 0RS" will find "EN35 0RS" and identify the country as United Kingdom
///
/// Note that some countries can not be uniquely identified, for example, if
/// they use a five digit postcode
pub fn evaluate_all_countries(
    haystack: &str,
    check_position: bool,
) -> Result<Option<PostcodeWrapper>, PostcodeError> {
    for country in UNIQUE_COUNTRIES.iter().cloned() {
        if let Ok(Some(pc)) = evaluate_single_country(haystack, country.clone(), check_position) {
            return Ok(Some(PostcodeWrapper {
                country: country,
                postcode: pc,
            }));
        }
    }

    // 5 Digits but with a space
    // We MIGHT be able to say it's Czechia if they include the CZ-
    if let Ok(Some(pc)) = evaluate_single_country(haystack, Country::CZ, check_position) {
        if pc.base.contains("CZ-") {
            return Ok(Some(PostcodeWrapper {
                country: Country::CZ,
                postcode: pc,
            }));
        }
    }

    // We MIGHT be able to say it's Greece if they include the GR-
    if let Ok(Some(pc)) = evaluate_single_country(haystack, Country::GR, check_position) {
        if pc.base.contains("GR-") {
            return Ok(Some(PostcodeWrapper {
                country: Country::GR,
                postcode: pc,
            }));
        }
    }

    // Generic 5 digit with space
    if let Ok(Some(pc)) =
        evaluate_single_country(haystack, Country::Unknown5DigitSpace, check_position)
    {
        return Ok(Some(PostcodeWrapper {
            country: Country::Unknown5DigitSpace,
            postcode: pc,
        }));
    }

    // USA and Saudi Arabia Special Case
    if let Ok(Some(pc)) = evaluate_single_country(haystack, Country::US, check_position) {
        if pc.additional.is_some() {
            return Ok(Some(PostcodeWrapper {
                country: Country::Unknown5DigitAdditional,
                postcode: pc,
            }));
        }
    }

    // Six digit
    if let Ok(Some(pc)) = evaluate_single_country(haystack, Country::Unknown6Digit, check_position)
    {
        return Ok(Some(PostcodeWrapper {
            country: Country::Unknown6Digit,
            postcode: pc,
        }));
    }

    // Taiwan HACK
    // only if we check position
    if check_position {
        if let Ok(Some(pc)) = evaluate_single_country(haystack, Country::TW, check_position) {
            return Ok(Some(PostcodeWrapper {
                country: Country::TW,
                postcode: pc,
            }));
        }
    }

    // 5 digit

    if let Ok(Some(pc)) = evaluate_single_country(haystack, Country::Unknown5Digit, check_position)
    {
        return Ok(Some(PostcodeWrapper {
            country: Country::Unknown5Digit,
            postcode: pc,
        }));
    }

    // Cyprus special case
    if let Ok(Some(pc)) = evaluate_single_country(haystack, Country::CY, check_position) {
        if pc.base.contains("CY-") {
            return Ok(Some(PostcodeWrapper {
                country: Country::CY,
                postcode: pc,
            }));
        }
    }
    // 4 digit

    if let Ok(Some(pc)) = evaluate_single_country(haystack, Country::Unknown4Digit, check_position)
    {
        return Ok(Some(PostcodeWrapper {
            country: Country::Unknown4Digit,
            postcode: pc,
        }));
    }
    return Ok(None);
}
