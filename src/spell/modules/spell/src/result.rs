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

impl From<eyre::Result<()>> for UnitResult {
    fn from(value: eyre::Result<()>) -> Self {
        match value {
            Ok(_) => UnitResult::ok(),
            Err(e) => UnitResult::error(e),
        }
    }
}