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
            Country::AT => todo!(),
            Country::AU => todo!(),
            Country::BE => todo!(),
            Country::BR => todo!(),
            Country::CA => todo!(),
            Country::CH => todo!(),
            Country::CY => todo!(),
            Country::CZ => todo!(),
            Country::DE => todo!(),
            Country::DK => todo!(),
            Country::EE => todo!(),
            Country::ES => todo!(),
            Country::FI => todo!(),
            Country::FR => todo!(),
            Country::GB => todo!(),
            Country::GR => todo!(),
            Country::HU => todo!(),
            Country::ID => todo!(),
            Country::IE => todo!(),
            Country::IN => todo!(),
            Country::IT => todo!(),
            Country::JP => todo!(),
            Country::KR => todo!(),
            Country::LT => todo!(),
            Country::LV => todo!(),
            Country::MX => todo!(),
            Country::MY => todo!(),
            Country::NL => todo!(),
            Country::NO => todo!(),
            Country::NZ => todo!(),
            Country::PE => todo!(),
            Country::PH => todo!(),
            Country::PL => todo!(),
            Country::PT => todo!(),
            Country::SE => todo!(),
            Country::SG => todo!(),
            Country::SK => todo!(),
            Country::TH => todo!(),
            Country::TW => todo!(),
            Country::US => todo!(),
            Country::VN => todo!(),
            Country::Unknown4Digit => todo!(),
            Country::Unknown5Digit => todo!(),
            Country::Unknown6Digit => todo!(),
            Country::Unknown5DigitSpace => todo!(),
        };
    }

    pub fn to_local_name(&self) -> String {
        return match self {
            Country::AT => todo!(),
            Country::AU => todo!(),
            Country::BE => todo!(),
            Country::BR => todo!(),
            Country::CA => todo!(),
            Country::CH => todo!(),
            Country::CY => todo!(),
            Country::CZ => todo!(),
            Country::DE => todo!(),
            Country::DK => todo!(),
            Country::EE => todo!(),
            Country::ES => todo!(),
            Country::FI => todo!(),
            Country::FR => todo!(),
            Country::GB => todo!(),
            Country::GR => todo!(),
            Country::HU => todo!(),
            Country::ID => todo!(),
            Country::IE => todo!(),
            Country::IN => todo!(),
            Country::IT => todo!(),
            Country::JP => todo!(),
            Country::KR => todo!(),
            Country::LT => todo!(),
            Country::LV => todo!(),
            Country::MX => todo!(),
            Country::MY => todo!(),
            Country::NL => todo!(),
            Country::NO => todo!(),
            Country::NZ => todo!(),
            Country::PE => todo!(),
            Country::PH => todo!(),
            Country::PL => todo!(),
            Country::PT => todo!(),
            Country::SE => todo!(),
            Country::SG => todo!(),
            Country::SK => todo!(),
            Country::TH => todo!(),
            Country::TW => todo!(),
            Country::US => todo!(),
            Country::VN => todo!(),
            Country::Unknown4Digit => todo!(),
            Country::Unknown5Digit => todo!(),
            Country::Unknown6Digit => todo!(),
            Country::Unknown5DigitSpace => todo!(),
        };
    }
}
