use regex::Regex;

use crate::{PostCodeParser, PostcodeHolder, is_more_than_30_percent_through_string};

const US_REGEX: &str = r"\b(\d{5})(?:[-+])?(\d{4})?\b";

/// Convenience wrapper for USA Regex
pub struct FiveDigitRegex(Regex);

impl FiveDigitRegex {
    /// Make a new USA ZIP Regex
    pub fn new() -> Self {
        Self(Regex::new(US_REGEX).expect("Could not create regex!"))
    }
}

impl PostCodeParser for FiveDigitRegex {
    fn evaluate(&self, haystack: &str, check_position: bool) -> Option<PostcodeHolder> {
        let captures = self.0.captures_iter(haystack);

        let last = captures.last()?;

        let cap1 = last.get(1)?;
        let zip = cap1.as_str().to_string();

        if check_position {
            if !is_more_than_30_percent_through_string(haystack, &cap1) {
                return None;
            }
        }

        let cap2 = last.get(2);
        let zip_p_4 = match cap2 {
            None => None,
            Some(z4) => Some(z4.as_str().to_string()),
        };

        return Some(PostcodeHolder {
            base: zip,
            additional: zip_p_4,
        });
    }
}
