use regex::Regex;

use crate::{PostCodeParser, PostcodeHolder, is_more_than_30_percent_through_string};

const NL_REGEX: &str = r"\b(\d{4}\s{0,1}[A-Za-z]{2})\b";

pub struct NlRegex(Regex);

impl NlRegex {
    pub fn new() -> Self {
        Self(Regex::new(NL_REGEX).expect("Could not create regex!"))
    }
}

impl PostCodeParser for NlRegex {
    fn evaluate(&self, haystack: &str, check_position: bool) -> Option<PostcodeHolder> {
        let postalcode_captures = self.0.captures_iter(haystack);

        let best_match = postalcode_captures.last()?.get(0)?;

        if check_position {
            if !is_more_than_30_percent_through_string(haystack, &best_match) {
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
