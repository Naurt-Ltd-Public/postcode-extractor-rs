use regex::Regex;

use crate::{PostCodeParser, PostcodeHolder, is_less_than_30_percent_through_string};

const JP_REGEX: &str = r"\b[〒〠⮗〶]?(\d{3}[ -]?\d{4})";

/// Convenience wrapper for USA Regex
pub struct JpRegex(Regex);

impl JpRegex {
    /// Make a new USA ZIP Regex
    pub fn new() -> Self {
        Self(Regex::new(JP_REGEX).expect("Could not create regex!"))
    }
}

impl PostCodeParser for JpRegex {
    fn evaluate(&self, haystack: &str, check_position: bool) -> Option<PostcodeHolder> {
        let postalcode_captures = self.0.captures_iter(haystack);

        let best_match = postalcode_captures.last()?.get(0)?;

        if check_position {
            if !is_less_than_30_percent_through_string(haystack, &best_match) {
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
