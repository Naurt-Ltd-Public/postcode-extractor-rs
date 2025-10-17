use regex::Regex;

use crate::{PostCodeParser, PostcodeHolder, is_more_than_50_percent_through_string};

const BR_REGEX: &str = r"\b(\d{5}-\d{3})\b";

/// Convenience wrapper for USA Regex
pub struct BrRegex(Regex);

impl BrRegex {
    /// Make a new USA ZIP Regex
    pub fn new() -> Self {
        Self(Regex::new(BR_REGEX).expect("Could not create regex!"))
    }
}

impl PostCodeParser for BrRegex {
    fn evaluate(&self, haystack: &str, check_position: bool) -> Option<PostcodeHolder> {
        let postalcode_captures = self.0.captures_iter(haystack);

        let best_match = postalcode_captures.last()?.get(0)?;

        if check_position {
            if !is_more_than_50_percent_through_string(haystack, &best_match) {
                return None;
            }
        }

        let best_match = best_match.as_str().to_string();
        Some(PostcodeHolder {
            base: best_match,
            additional: None,
        })
    }
}
