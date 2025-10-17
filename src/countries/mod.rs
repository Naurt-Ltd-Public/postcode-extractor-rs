#[cfg(test)]
mod tests;

mod brazil;
mod canada;
mod five_digit;
mod four_digit;
mod japan;
mod latvia;
mod netherlands;
mod portugal;
mod six_digit;
mod uk;

pub use brazil::BrRegex;
pub use canada::CaRegex;
pub use five_digit::FiveDigitRegex;
pub use four_digit::FourRegex;
pub use japan::JpRegex;
pub use latvia::LvRegex;
pub use netherlands::NlRegex;
pub use portugal::PtRegex;
pub use six_digit::SixRegex;
pub use uk::UkRegex;
