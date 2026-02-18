use serde::{Deserialize, Serialize};

/// Countries which use four digits in the postcode, e.g. `1234`
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
    Country::NO,
    Country::LU,
    Country::SI,
    Country::ZA,
];

/// Countries which use five digits in the postcode, e.g. `12345`
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
    Country::EG,
    Country::SA,
    Country::AE,
];

/// Countries which use six digits in the postcode, e.g. `123456`
pub static SIX_DIGIT_NATIONS: &[Country] = &[Country::SG, Country::IN, Country::TW];

/// Countries which use five digits in the postcode, but there's a mandatory space, e.g. `653 02`
pub static FIVE_DIGIT_WITH_SPACE_NATIONS: &[Country] =
    &[Country::GR, Country::CZ, Country::SE, Country::SK];

/// Countries which have a unique enough postcode to be identified e.g. UK and the SW17 0QN
pub static UNIQUE_COUNTRIES: &[Country] = &[
    Country::GB,
    Country::IM,
    Country::JE,
    Country::GG,
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

pub static FIVE_DIGIT_ADDITIONAL_NATIONS: &[Country] = &[Country::US, Country::SA];

/// All countries supported by the postcode extractor
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum Country {
    /// UAE
    AE,
    /// Austria
    AT,
    /// Australia
    AU,
    /// Belgium
    BE,
    /// Brazil
    BR,
    /// Canada
    CA,
    /// Switzerland
    CH,
    /// Cyprus
    CY,
    /// Czechia
    CZ,
    /// Germany
    DE,
    /// Denmark
    DK,
    /// Estonia
    EE,
    /// Egypt
    EG,
    /// Spain
    ES,
    /// Finland
    FI,
    /// France
    FR,
    /// UK
    GB,
    /// Guernsey
    GG,
    /// Greece
    GR,
    /// Hungary
    HU,
    /// Indonesia
    ID,
    /// Ireland
    IE,
    /// Isle of Man
    IM,
    /// India
    IN,
    /// Italy
    IT,
    /// Japan
    JP,
    /// Jersey
    JE,
    /// Korea, Republic of. (South Korea)
    KR,
    /// Lithuania
    LT,
    /// Luxembourg
    LU,
    /// Latvia
    LV,
    /// Mexico
    MX,
    /// Malaysia
    MY,
    /// Netherlands
    NL,
    /// Norway
    NO,
    /// New Zealand
    NZ,
    /// Peru
    PE,
    /// Philippines
    PH,
    /// Poland
    PL,
    /// Portugal
    PT,
    /// Saudi Arabia
    SA,
    /// Sweden
    SE,
    /// Singapore
    SG,
    /// Slovenia
    SI,
    /// Slovakia
    SK,
    /// Thailand
    TH,
    /// Taiwan
    TW,
    /// USA
    US,
    /// Vietnam
    VN,
    /// Hong Kong
    HK,
    /// South Africa
    ZA,
    /// Countries which use four digits in the postcode, e.g. `1234`
    Unknown4Digit,
    /// Countries which use five digits in the postcode, e.g. `12345`
    Unknown5Digit,
    /// Countries which use six digits in the postcode, e.g. `123456`
    Unknown6Digit,
    /// Countries which use five digits in the postcode, but there's a mandatory space, e.g. `653 02`
    Unknown5DigitSpace,
    /// Countries with 5 digits and an additional 4 optional e.g. `12345-1234`
    Unknown5DigitAdditional,
}

impl Country {
    /// Convert a country into its English name
    pub fn to_en_name(&self) -> String {
        return match self {
            Country::AE => "United Arab Emirates".to_string(),
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
            Country::EG => "Egypt".to_string(),
            Country::ES => "Spain".to_string(),
            Country::FI => "Finland".to_string(),
            Country::FR => "France".to_string(),
            Country::GB => "United Kingdom".to_string(),
            Country::GG => "Guernsey".to_string(),
            Country::GR => "Greece".to_string(),
            Country::HU => "Hungary".to_string(),
            Country::ID => "Indonesia".to_string(),
            Country::IE => "Ireland".to_string(),
            Country::IM => "Isle of Man".to_string(),
            Country::IN => "India".to_string(),
            Country::IT => "Italy".to_string(),
            Country::JP => "Japan".to_string(),
            Country::JE => "Jersey".to_string(),
            Country::KR => "South Korea".to_string(),
            Country::LT => "Lithuania".to_string(),
            Country::LU => "Luxembourg".to_string(),
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
            Country::SA => "Saudi Arabia".to_string(),
            Country::SE => "Sweden".to_string(),
            Country::SG => "Singapore".to_string(),
            Country::SI => "Slovenia".to_string(),
            Country::SK => "Slovakia".to_string(),
            Country::TH => "Thailand".to_string(),
            Country::TW => "Taiwan".to_string(),
            Country::US => "United States".to_string(),
            Country::VN => "Vietnam".to_string(),
            Country::HK => "Hong Kong".to_string(),
            Country::ZA => "South Africa".to_string(),
            Country::Unknown4Digit => "An Unknown Country With a 4 Digit Postcode".to_string(),
            Country::Unknown5Digit => "An Unknown Country With a 5 Digit Postcode".to_string(),
            Country::Unknown6Digit => "An Unknown Country With a 6 Digit Postcode".to_string(),
            Country::Unknown5DigitSpace => {
                "An Unknown Country With a 5 Digit Postcode and Space".to_string()
            }
            Country::Unknown5DigitAdditional => {
                "An Unknown Country With a 5 Digit Plus 4 Digit Postcode".to_string()
            }
        };
    }

    /// Convert a country into its best fit local name
    pub fn to_local_name(&self) -> String {
        return match self {
            Country::AE => "الإمارات العربية المتحدة".to_string(),
            Country::AT => "Österreich".to_string(),
            Country::AU => "Australia".to_string(),
            Country::BE => "België".to_string(),
            Country::BR => "Brasil".to_string(),
            Country::CA => "Canada".to_string(),
            Country::CH => "Suisse".to_string(),
            Country::CY => "κύπρος".to_string(),
            Country::CZ => "Czechia".to_string(),
            Country::DE => "Deutschland".to_string(),
            Country::DK => "Danmark".to_string(),
            Country::EE => "Eesti".to_string(),
            Country::EG => "مصر".to_string(),
            Country::ES => "España".to_string(),
            Country::FI => "Suomi".to_string(),
            Country::FR => "France".to_string(),
            Country::GB => "United Kingdom".to_string(),
            Country::GG => "Guernsey".to_string(),
            Country::GR => "Ελλάδα".to_string(),
            Country::HU => "Magyarország".to_string(),
            Country::ID => "Indonesia".to_string(),
            Country::IE => "Ireland".to_string(),
            Country::IM => "Isle of Man".to_string(),
            Country::IN => "India".to_string(),
            Country::IT => "Italia".to_string(),
            Country::JE => "Jersey".to_string(),
            Country::JP => "日本".to_string(),
            Country::KR => "대한민국".to_string(),
            Country::LT => "Lithuania".to_string(),
            Country::LU => "Luxembourg".to_string(),
            Country::LV => "Latvia".to_string(),
            Country::MX => "Mexico".to_string(),
            Country::MY => "Malaysia".to_string(),
            Country::NL => "Nederland".to_string(),
            Country::NO => "Norge".to_string(),
            Country::NZ => "New Zealand".to_string(),
            Country::PE => "Perú".to_string(),
            Country::PH => "Philippines".to_string(),
            Country::PL => "Poland".to_string(),
            Country::PT => "Portugal".to_string(),
            Country::SA => "السعودية".to_string(),
            Country::SE => "Sverige".to_string(),
            Country::SG => "Singapore".to_string(),
            Country::SI => "Slovenia".to_string(),
            Country::SK => "Slovakia".to_string(),
            Country::TH => "ประเทศไทย".to_string(),
            Country::TW => "台灣".to_string(),
            Country::US => "United States".to_string(),
            Country::VN => "Việt Nam".to_string(),
            Country::HK => "香港".to_string(),
            Country::ZA => "South Africa".to_string(),
            Country::Unknown4Digit => "An Unknown Country With a 4 Digit Postcode".to_string(),
            Country::Unknown5Digit => "An Unknown Country With a 5 Digit Postcode".to_string(),
            Country::Unknown6Digit => "An Unknown Country With a 6 Digit Postcode".to_string(),
            Country::Unknown5DigitSpace => {
                "An Unknown Country With a 5 Digit Postcode and Space".to_string()
            }
            Country::Unknown5DigitAdditional => {
                "An Unknown Country With a 5 Digit Plus 4 Digit Postcode".to_string()
            }
        };
    }
}
