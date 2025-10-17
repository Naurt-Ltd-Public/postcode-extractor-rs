use regex::Regex;

use crate::{PostCodeParser, PostcodeHolder, is_more_than_50_percent_through_string};

const CA_REGEX: &str = r"\b[ABCEGHJ-NPRSTVXY]\d[ABCEGHJ-NPRSTV-Z][ -]?\d[ABCEGHJ-NPRSTV-Z]\d\b";

/// Convenience wrapper for creating a UK Post Code regex
pub struct CaRegex(Regex);

impl CaRegex {
    pub fn new() -> Self {
        Self(Regex::new(CA_REGEX).expect("Could not create regex!"))
    }
}

impl PostCodeParser for CaRegex {
    fn evaluate(&self, haystack: &str, check_position: bool) -> Option<PostcodeHolder> {
        let postalcode_captures = self.0.captures_iter(haystack);

        let best_match = postalcode_captures.last()?.get(0)?;

        if check_position {
            if !is_more_than_50_percent_through_string(haystack, &best_match) {
                return None;
            }
        }

        let mut uppercase_postalcode = best_match.as_str().to_uppercase();

        if uppercase_postalcode.len() > 3
            && !uppercase_postalcode
                .chars()
                .nth(uppercase_postalcode.len() - 4)?
                .is_whitespace()
        {
            uppercase_postalcode.insert(uppercase_postalcode.len() - 3, ' ');
        }

        Some(PostcodeHolder {
            base: uppercase_postalcode,
            additional: None,
        })
    }
}
