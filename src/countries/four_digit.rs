use regex::Regex;

use crate::{PostCodeParser, PostcodeHolder, is_more_than_50_percent_through_string};

const FOUR_REGEX: &str = r"\b(\d{4})\b";

/// Convenience wrapper for creating a Four Post Code regex
pub struct FourRegex(Regex);

impl FourRegex {
    /// Make a new Four postalcode regex
    pub fn new() -> Self {
        Self(Regex::new(FOUR_REGEX).expect("Could not create regex!"))
    }
}

impl PostCodeParser for FourRegex {
    fn evaluate(&self, haystack: &str, check_position: bool) -> Option<PostcodeHolder> {
        let postalcode_captures = self.0.captures_iter(haystack);

        let best_match = postalcode_captures.last()?.get(0)?;

        if check_position {
            if !is_more_than_50_percent_through_string(haystack, &best_match) {
                return None;
            }
        }

        let best_match = best_match.as_str().to_string();

        return Some(PostcodeHolder {
            base: best_match,
            additional: None,
        });
    }
}
