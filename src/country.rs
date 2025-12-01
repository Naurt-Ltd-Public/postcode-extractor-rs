use serde::{Deserialize, Serialize};

pub static FOUR_DIGIT_NATIONS: &[Country] = &[
    Country::AU,
    Country::NZ,
    Country::PH,
    Country::DK,
    Country::AT,
    Country::CH,
    Country::BE,
    Country::CY,
    Country::HU,
    Country::EE,
    Country::NO,
];

pub static FIVE_DIGIT_NATIONS: &[Country] = &[
    Country::US,
    Country::PE,
    Country::FR,
    Country::DE,
    Country::IT,
    Country::ID,
    Country::VN,
    Country::TH,
    Country::KR,
    Country::TW,
    Country::EE,
    Country::ES,
    Country::FI,
    Country::MY,
    Country::MX,
];

pub static SIX_DIGIT_NATIONS: &[Country] = &[Country::SG, Country::IN, Country::TW];

pub static FIVE_DIGIT_WITH_SPACE_NATIONS: &[Country] =
    &[Country::GR, Country::CZ, Country::SE, Country::SK];

pub static UNIQUE_COUNTRIES: &[Country] = &[
    Country::GB,
    Country::CA,
    Country::PT,
    Country::LT,
    Country::LV,
    Country::BR,
    Country::PL,
    Country::NL,
    Country::JP,
    Country::IE,
];

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum Country {
    AT, // Austria
    AU, // AUstralia
    BE, // Belgium
    BR, // Brazil
    CA, // Canada
    CH, // Switzerland
    CY, // Cyprus
    CZ, // Czechia
    DE, // Germany
    DK, // Denmark
    EE, // Estonia
    ES, // Spain
    FI, // Finland
    FR, // France
    GB, // UK
    GR, // Greece
    HU, // Hungary
    ID, // Indonesia
    IE, // Ireland
    IN, // India
    IT, // Italy
    JP, // Japan
    KR, // Korea, Republic of. (South Korea)
    LT, // Lithuania
    LV, // Latvia
    MX, // Mexico
    MY, // Malaysia
    NL, // Netherlands
    NO, // Norway
    NZ, // New Zealand
    PE, // Peru
    PH, // Philippines
    PL, // Poland
    PT, // Portugal
    SE, // Sweden
    SG, // Singapore
    SK, // Slovakia
    TH, // Thailand
    TW, // Taiwan
    US, // USA
    VN, // Vietnam
    Unknown4Digit,
    Unknown5Digit,
    Unknown6Digit,
    Unknown5DigitSpace,
}

impl Country {
    pub fn to_en_name(&self) -> String {
        return match self {
            Country::AT => "Austria".to_string(),
            Country::AU => "Australia".to_string(),
            Country::BE => "Belgium".to_string(),
            Country::BR => "Brazil".to_string(),
            Country::CA => "Canada".to_string(),
            Country::CH => "Switzerland".to_string(),
            Country::CY => "Cyprus".to_string(),
            Country::CZ => "Czechia".to_string(),
            Country::DE => "Germany".to_string(),
            Country::DK => "Denmark".to_string(),
            Country::EE => "Estonia".to_string(),
            Country::ES => "Spain".to_string(),
            Country::FI => "Finland".to_string(),
            Country::FR => "France".to_string(),
            Country::GB => "United Kingdom".to_string(),
            Country::GR => "Greece".to_string(),
            Country::HU => "Hungary".to_string(),
            Country::ID => "Indonesia".to_string(),
            Country::IE => "Ireland".to_string(),
            Country::IN => "India".to_string(),
            Country::IT => "Italy".to_string(),
            Country::JP => "Japan".to_string(),
            Country::KR => "South Korea".to_string(),
            Country::LT => "Lithuania".to_string(),
            Country::LV => "Latvia".to_string(),
            Country::MX => "Mexico".to_string(),
            Country::MY => "Malaysia".to_string(),
            Country::NL => "Netherlands".to_string(),
            Country::NO => "Norway".to_string(),
            Country::NZ => "New Zealand".to_string(),
            Country::PE => "Peru".to_string(),
            Country::PH => "Philippines".to_string(),
            Country::PL => "Poland".to_string(),
            Country::PT => "Portugal".to_string(),
            Country::SE => "Sweden".to_string(),
            Country::SG => "Singapore".to_string(),
            Country::SK => "Slovakia".to_string(),
            Country::TH => "Thailand".to_string(),
            Country::TW => "Taiwan".to_string(),
            Country::US => "United States".to_string(),
            Country::VN => "Vietnam".to_string(),
            Country::Unknown4Digit => "An Unknown Country With a 4 Digit Postcode".to_string(),
            Country::Unknown5Digit => "An Unknown Country With a 5 Digit Postcode".to_string(),
            Country::Unknown6Digit => "An Unknown Country With a 6 Digit Postcode".to_string(),
            Country::Unknown5DigitSpace => {
                "An Unknown Country With a 5 Digit Postcode and Space".to_string()
            }
        };
    }

    pub fn to_local_name(&self) -> String {
        return match self {
            Country::AT => "Österreich".to_string(),
            Country::AU => "Australia".to_string(),
            Country::BE => "België".to_string(),
            Country::BR => "Brasil".to_string(),
            Country::CA => "Canada".to_string(),
            Country::CH => "Switzerland".to_string(),
            Country::CY => "κύπρος".to_string(),
            Country::CZ => "Czechia".to_string(),
            Country::DE => "Deutschland".to_string(),
            Country::DK => "Danmark".to_string(),
            Country::EE => "Eesti".to_string(),
            Country::ES => "España".to_string(),
            Country::FI => "Suomen".to_string(),
            Country::FR => "France".to_string(),
            Country::GB => "United Kingdom".to_string(),
            Country::GR => "Ελλάδα".to_string(),
            Country::HU => "Magyarország".to_string(),
            Country::ID => "Indonesia".to_string(),
            Country::IE => "Ireland".to_string(),
            Country::IN => "India".to_string(),
            Country::IT => "Italia".to_string(),
            Country::JP => "日本".to_string(),
            Country::KR => "대한민국".to_string(),
            Country::LT => "Lithuania".to_string(),
            Country::LV => "Latvia".to_string(),
            Country::MX => "Mexico".to_string(),
            Country::MY => "Malaysia".to_string(),
            Country::NL => "Nederland".to_string(),
            Country::NO => "Norway".to_string(),
            Country::NZ => "New Zealand".to_string(),
            Country::PE => "Perú".to_string(),
            Country::PH => "Philippines".to_string(),
            Country::PL => "Poland".to_string(),
            Country::PT => "Portugal".to_string(),
            Country::SE => "Sweden".to_string(),
            Country::SG => "Singapore".to_string(),
            Country::SK => "Slovakia".to_string(),
            Country::TH => "ประเทศไทย".to_string(),
            Country::TW => "台灣".to_string(),
            Country::US => "United States".to_string(),
            Country::VN => "Việt Nam".to_string(),
            Country::Unknown4Digit => "An Unknown Country With a 4 Digit Postcode".to_string(),
            Country::Unknown5Digit => "An Unknown Country With a 5 Digit Postcode".to_string(),
            Country::Unknown6Digit => "An Unknown Country With a 6 Digit Postcode".to_string(),
            Country::Unknown5DigitSpace => {
                "An Unknown Country With a 5 Digit Postcode and Space".to_string()
            }
        };
    }
}
