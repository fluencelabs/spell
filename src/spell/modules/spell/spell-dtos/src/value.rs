use marine_rs_sdk::{marine, CallParameters};

use crate::error::SpellError;

pub fn format_error(e: impl std::fmt::Debug) -> String {
    format!("{:?}", e)
}

#[marine]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UnitValue {
    pub success: bool,
    pub error: String,
}

impl UnitValue {
    pub fn ok() -> Self {
        Self {
            success: true,
            error: <_>::default(),
        }
    }

    pub fn error(error: impl AsRef<str>) -> Self {
        Self {
            success: false,
            error: error.as_ref().to_string(),
        }
    }

    pub fn spell_error(error: SpellError) -> Self {
        Self::error(format_error(error))
    }
}

impl From<eyre::Result<()>> for UnitValue {
    fn from(value: eyre::Result<()>) -> Self {
        match value {
            Ok(_) => UnitValue::ok(),
            Err(e) => UnitValue::error(format_error(e)),
        }
    }
}

impl From<SpellError> for UnitValue {
    fn from(error: SpellError) -> Self {
        UnitValue::spell_error(error)
    }
}

#[marine]
pub struct StringValue {
    pub str: String,
    pub success: bool,
    pub error: String,
}

impl From<eyre::Result<String>> for StringValue {
    fn from(value: eyre::Result<String>) -> Self {
        match value {
            Ok(str) => StringValue {
                str,
                success: true,
                error: <_>::default(),
            },
            Err(e) => StringValue {
                str: <_>::default(),
                success: false,
                error: format_error(e),
            },
        }
    }
}

#[marine]
pub struct StringListValue {
    pub strings: Vec<String>,
    pub success: bool,
    pub error: String,
}

impl From<eyre::Result<Vec<String>>> for StringListValue {
    fn from(value: eyre::Result<Vec<String>>) -> Self {
        match value {
            Ok(strings) => StringListValue {
                strings,
                success: true,
                error: <_>::default(),
            },
            Err(e) => StringListValue {
                strings: <_>::default(),
                success: false,
                error: format_error(e),
            },
        }
    }
}

#[marine]
pub struct U32Value {
    pub num: u32,
    pub success: bool,
    pub error: String,
}

impl From<eyre::Result<u32>> for U32Value {
    fn from(value: eyre::Result<u32>) -> Self {
        match value {
            Ok(num) => U32Value {
                num,
                success: true,
                error: <_>::default(),
            },
            Err(e) => U32Value {
                num: <_>::default(),
                success: false,
                error: format_error(e),
            },
        }
    }
}

#[marine]
pub struct LocationValue {
    pub relay: String,
    pub host: String,
    pub service_id: String,

    pub success: bool,
    pub error: String,
}

impl LocationValue {
    pub fn error(error: eyre::Report) -> Self {
        Self {
            relay: <_>::default(),
            host: <_>::default(),
            service_id: <_>::default(),
            success: false,
            error: format_error(error),
        }
    }

    pub fn success(relay: String, params: CallParameters) -> Self {
        Self {
            relay,
            host: params.host_id,
            service_id: params.service_id,
            success: true,
            error: <_>::default(),
        }
    }
}
