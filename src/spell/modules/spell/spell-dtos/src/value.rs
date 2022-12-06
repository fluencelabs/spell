use marine_rs_sdk::{CallParameters, marine};
use serde::Deserialize;

use crate::error::SpellError;

pub fn format_error(e: impl std::fmt::Debug) -> String {
    format!("{:?}", e)
}

pub trait SpellValueT {
    fn is_success(&self) -> bool;
    fn get_error(&self) -> String;
}

#[marine]
#[derive(Debug, Clone, Deserialize)]
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

impl SpellValueT for UnitValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn get_error(&self) -> String {
        self.error.clone()
    }
}

impl From<SpellError> for UnitValue {
    fn from(error: SpellError) -> Self {
        UnitValue::spell_error(error)
    }
}

#[marine]
#[derive(Deserialize)]
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

impl SpellValueT for StringValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn get_error(&self) -> String {
        self.error.clone()
    }
}

#[marine]
#[derive(Deserialize)]
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

impl SpellValueT for StringListValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn get_error(&self) -> String {
        self.error.clone()
    }
}

#[marine]
#[derive(Deserialize)]
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

impl SpellValueT for U32Value {
    fn is_success(&self) -> bool {
        self.success
    }

    fn get_error(&self) -> String {
        self.error.clone()
    }
}

#[marine]
#[derive(Deserialize)]
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

impl SpellValueT for LocationValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn get_error(&self) -> String {
        self.error.clone()
    }
}

#[marine]
#[derive(Deserialize)]
pub struct ScriptValue {
    pub source_code: String,
    pub success: bool,
    pub error: String,
}

impl SpellValueT for ScriptValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn get_error(&self) -> String {
        self.error.clone()
    }
}

#[marine]
#[derive(Deserialize)]
pub struct CIDValue {
    pub v1_str: String,
    pub success: bool,
    pub error: String,
}

impl SpellValueT for CIDValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn get_error(&self) -> String {
        self.error.clone()
    }
}
