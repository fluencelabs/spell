use marine_rs_sdk::marine;
use std::fmt::Display;

#[marine]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UnitResult {
    success: bool,
    error: String,
}

impl UnitResult {
    pub fn ok() -> Self {
        Self {
            success: true,
            error: <_>::default(),
        }
    }

    pub fn error(error: impl Display) -> Self {
        Self {
            success: false,
            error: error.to_string(),
        }
    }
}
