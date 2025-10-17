use std::fmt::Display;

use regex::Match;

mod countries;
mod country_enums;
mod statics;

use crate::countries::{
    BrRegex, CaRegex, FiveDigitRegex, FourRegex, JpRegex, LvRegex, NlRegex, PtRegex, SixRegex,
    UkRegex,
};

// TODO: Move this elsewhere
pub static FIVE_DIGIT_NATIONS: &'static [&str] = &[
    "United States",
    "Perú",
    "France",
    "Deutschland",
    "Italia",
    "Indonesia",
    "ประเทศไทย",
    "Việt Nam",
    "대한민국", // South Korea
    "Mongolia",
];
pub static FOUR_DIGIT_NATIONS: &'static [&str; 6] = &[
    "Australia",
    "New Zealand",
    "Philippines",
    "Denmark",
    "Austria",
    "Switzerland",
];
pub static SIX_DIGIT_NATIONS: &'static [&str] = &[
    "Singapore",
    "India",
    "台灣", //Taiwan
];

enum Percent {
    Fifty,
    Forty,
    Thirty,
}

impl Percent {
    fn to_num(&self) -> f64 {
        match self {
            Self::Fifty => 0.5,
            Self::Forty => 0.4,
            Self::Thirty => 0.3,
        }
    }
}

// TODO: sort this stuff out it's frankly shocking!
fn is_more_than_50_percent_through_string(haystack: &str, mat: &Match) -> bool {
    return is_more_than_x_percent_through_string(haystack, mat, Percent::Fifty);
}

fn is_more_than_30_percent_through_string(haystack: &str, mat: &Match) -> bool {
    return is_more_than_x_percent_through_string(haystack, mat, Percent::Thirty);
}

fn is_less_than_30_percent_through_string(haystack: &str, mat: &Match) -> bool {
    return is_less_than_x_percent_through_string(haystack, mat, Percent::Thirty);
}

fn is_more_than_x_percent_through_string(haystack: &str, mat: &Match, percent: Percent) -> bool {
    let hay_len = haystack.chars().count() as f64;
    let match_char_idex = haystack[..mat.start()].chars().count() as f64;

    return if match_char_idex / hay_len >= percent.to_num() {
        true
    } else {
        false
    };
}

fn is_less_than_x_percent_through_string(haystack: &str, mat: &Match, percent: Percent) -> bool {
    let hay_len = haystack.chars().count() as f64;
    let match_char_idex = haystack[..mat.start()].chars().count() as f64;

    return if match_char_idex / hay_len <= percent.to_num() {
        true
    } else {
        false
    };
}

/// Enum for all possible postalcodes for each supported city
#[derive(Debug)]
pub enum PostalCodes {
    /// United Kingdom
    UK(String),
    /// United States of America
    US(PostcodeHolder),
    /// Latvia
    Lv(String),
    /// Canada
    CA(String),
    /// Brazil
    BR(String),
    /// Portugall
    PT(String),
    /// Netherlands
    NL(String),
    /// Japan
    JP(String),
    /// An unknown country with a 5 digit postalcode. Either USA or Peru right now
    Unknown5Digit(String),
    /// An unknown country with a 4 digit postalcode: AU, NZ, PH
    Unknown4Digit(String),
    /// An unkown country with a 6 digit postalcode: IN, SG
    Unknown6Digit(String),
}

impl PostalCodes {
    pub fn get_inner<'a>(&'a self) -> &'a str {
        return match self {
            PostalCodes::UK(uk) => uk,
            PostalCodes::US(uspostal) => &uspostal.base,
            PostalCodes::Lv(lv) => lv,
            PostalCodes::CA(ca) => ca,
            PostalCodes::BR(br) => br,
            PostalCodes::Unknown5Digit(five) => five,
            PostalCodes::Unknown4Digit(four) => four,
            PostalCodes::Unknown6Digit(six) => six,
            PostalCodes::PT(pt) => pt,
            PostalCodes::NL(nl) => nl,
            PostalCodes::JP(jp) => jp,
        };
    }
}

/// Convenience type that represents a regex for each country
///
/// These regexes pull the postal code out of an address string
pub struct PostalCodeRegexes {
    pub uk: UkRegex,
    pub ca: CaRegex,
    pub five: FiveDigitRegex,
    pub lv: LvRegex,
    pub br: BrRegex,
    pub four: FourRegex,
    pub six: SixRegex,
    pub pt: PtRegex,
    pub nl: NlRegex,
    pub jp: JpRegex,
}

impl PostalCodeRegexes {
    /// Create regexes for each country which find the postalcode from an address
    /// string
    pub fn new() -> Self {
        return Self {
            uk: UkRegex::new(),
            ca: CaRegex::new(),
            five: FiveDigitRegex::new(),
            lv: LvRegex::new(),
            br: BrRegex::new(),
            four: FourRegex::new(),
            six: SixRegex::new(),
            pt: PtRegex::new(),
            nl: NlRegex::new(),
            jp: JpRegex::new(),
        };
    }

    /// Given an address string, this function checks to see if it can match that
    /// address string against any of the regexes. If it is able to, it returns the
    /// postalcode, as well as the country with the matching postalcode
    pub fn extract_postalcode_and_country(
        &self,
        haystack: &str,
        check_position: bool,
    ) -> Option<PostalCodes> {
        // UK
        if let Some(uk_postalcode) = self.uk.evaluate(haystack, check_position) {
            return Some(PostalCodes::UK(uk_postalcode.base));
        }

        // Canada
        if let Some(ca_postalcode) = self.ca.evaluate(haystack, check_position) {
            return Some(PostalCodes::CA(ca_postalcode.base));
        }

        // Japan
        if let Some(jp_postcode) = self.jp.evaluate(haystack, check_position) {
            return Some(PostalCodes::JP(jp_postcode.base));
        }

        // Brazil
        if let Some(br_postalcode) = self.br.evaluate(haystack, check_position) {
            return Some(PostalCodes::BR(br_postalcode.base));
        }

        // Latvia
        if let Some(lv_pc) = self.lv.evaluate(haystack, check_position) {
            return Some(PostalCodes::Lv(lv_pc.base));
        }

        // USA, Peru and all 5 digit postal codes
        // We reuse the USA regex for brevity, but if no ZIP+4 is found, all you
        // can say is it is a 5-digit nation
        if let Some(five_pc) = self.five.evaluate(haystack, check_position) {
            if five_pc.additional.is_none() {
                return Some(PostalCodes::Unknown5Digit(five_pc.base));
            } else {
                return Some(PostalCodes::US(five_pc));
            }
        }

        // Portugal
        if let Some(pt_postalcode) = self.pt.evaluate(haystack, check_position) {
            return Some(PostalCodes::PT(pt_postalcode.base));
        }

        // Netherlands
        if let Some(nl_postcode) = self.nl.evaluate(haystack, check_position) {
            return Some(PostalCodes::NL(nl_postcode.base));
        }

        // Six Digit
        if let Some(six_postcode) = self.six.evaluate(haystack, check_position) {
            return Some(PostalCodes::Unknown6Digit(six_postcode.base));
        }

        // Four digits
        if let Some(four_pc) = self.four.evaluate(haystack, check_position) {
            return Some(PostalCodes::Unknown4Digit(four_pc.base));
        }

        // Nothing

        return None;
    }
}

/// Struct for holding the USA ZIP
#[derive(Debug)]
pub struct PostcodeHolder {
    /// The ZIP stem i.e. 12345. ALWAYS 5 digits
    pub base: String,
    /// The +4. ALWAYS 4 digits e.g. 1234. This should NEVER include the - or +
    ///
    /// Optional as people don't always include them
    pub additional: Option<String>,
}

impl Display for PostcodeHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return if let Some(a) = &self.additional {
            write!(f, "{}+{}", self.base, a)
        } else {
            write!(f, "{}", self.base)
        };
    }
}

pub trait PostCodeParser {
    fn evaluate(&self, haystack: &str, check_position: bool) -> Option<PostcodeHolder>;
}
