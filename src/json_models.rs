use serde::Deserialize;

use crate::country::Country;

#[derive(Debug, Deserialize)]
pub struct RegexJson {
    pub country: Country,
    pub comment: String,
    pub priority: u8,
    pub regex: RegexData,
}

#[derive(Debug, Deserialize)]
pub struct RegexData {
    pub engines: RegexEngines,
    pub position_logic: PositionLogic,
}

#[derive(Debug, Deserialize)]
pub struct RegexEngines {
    pub rust: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PositionLogic {
    pub operation: PositionOperation,
    pub position: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "lowercase")]
pub enum PositionOperation {
    GT,
    GTE,
    LT,
    LTE,
}

impl PositionOperation {
    pub fn as_function(&self) -> fn(f64, f64) -> bool {
        return match self {
            PositionOperation::GT => |x, y| x > y,
            PositionOperation::GTE => |x, y| x >= y,
            PositionOperation::LT => |x, y| x < y,
            PositionOperation::LTE => |x, y| x <= y,
        };
    }
}
