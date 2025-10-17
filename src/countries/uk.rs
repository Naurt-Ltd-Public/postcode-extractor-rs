use regex::Regex;

use crate::{PostCodeParser, PostcodeHolder, is_more_than_50_percent_through_string};

const UK_REGEX: &str =
    r"\b(?:[A-Za-z][A-HJ-Ya-hj-y]?[0-9][0-9A-Za-z]? ?[0-9][A-Za-z]{2}|[Gg][Ii][Rr] ?0[Aa]{2})\b";

/// Convenience wrapper for creating a UK Post Code regex
pub struct UkRegex(Regex);

impl UkRegex {
    /// Make a new UK postalcode regex
    pub fn new() -> Self {
        Self(Regex::new(UK_REGEX).expect("Could not create regex!"))
    }
}

impl PostCodeParser for UkRegex {
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

        return Some(PostcodeHolder {
            base: uppercase_postalcode,
            additional: None,
        });
    }
}
