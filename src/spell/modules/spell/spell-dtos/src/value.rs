use marine_rs_sdk::{marine, CallParameters};
use marine_sqlite_connector::Statement;
use serde::Deserialize;

use crate::error::SpellError;

pub fn format_error(e: impl std::fmt::Debug) -> String {
    format!("{:?}", e)
}

pub trait SpellValueT {
    fn is_success(&self) -> bool;
    fn take_error(self) -> String;
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
            error: String::new(),
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

    fn take_error(self) -> String {
        self.error
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
    pub absent: bool,
}

impl From<eyre::Result<Option<String>>> for StringValue {
    fn from(value: eyre::Result<Option<String>>) -> Self {
        match value {
            Ok(Some(str)) => StringValue {
                str,
                success: true,
                error: String::new(),
                absent: false,
            },
            Ok(None) => StringValue {
                str: String::new(),
                success: true,
                error: String::new(),
                absent: true,
            },
            Err(e) => StringValue {
                str: String::new(),
                success: false,
                error: format_error(e),
                absent: false,
            },
        }
    }
}

impl SpellValueT for StringValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn take_error(self) -> String {
        self.error
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
                error: String::new(),
            },
            Err(e) => StringListValue {
                strings: vec![],
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

    fn take_error(self) -> String {
        self.error
    }
}

#[marine]
#[derive(Deserialize)]
pub struct U32Value {
    pub num: u32,
    pub success: bool,
    pub error: String,
    pub absent: bool,
}

impl From<eyre::Result<Option<u32>>> for U32Value {
    fn from(value: eyre::Result<Option<u32>>) -> Self {
        match value {
            Ok(Some(num)) => U32Value {
                num,
                success: true,
                error: String::new(),
                absent: false,
            },
            Ok(None) => U32Value {
                num: u32::default(),
                success: true,
                error: String::new(),
                absent: true,
            },
            Err(e) => U32Value {
                num: u32::default(),
                success: false,
                error: format_error(e),
                absent: false,
            },
        }
    }
}

impl SpellValueT for U32Value {
    fn is_success(&self) -> bool {
        self.success
    }

    fn take_error(self) -> String {
        self.error
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
            relay: String::new(),
            host: String::new(),
            service_id: String::new(),
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
            error: String::new(),
        }
    }
}

impl SpellValueT for LocationValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn take_error(self) -> String {
        self.error
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

    fn take_error(self) -> String {
        self.error
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

    fn take_error(self) -> String {
        self.error
    }
}

#[marine]
#[derive(Deserialize)]
pub struct BoolValue {
    pub flag: bool,
    pub success: bool,
    pub error: String,
}

impl SpellValueT for BoolValue {
    fn is_success(&self) -> bool {
        self.success
    }

    fn take_error(self) -> String {
        self.error
    }
}

impl From<eyre::Result<bool>> for BoolValue {
    fn from(value: eyre::Result<bool>) -> Self {
        match value {
            Ok(flag) => BoolValue {
                flag,
                success: true,
                error: String::new(),
            },
            Err(e) => BoolValue {
                flag: false,
                success: false,
                error: format_error(e),
            },
        }
    }
}

#[marine]
pub struct MailboxMessage {
    pub init_peer_id: String,
    pub timestamp: u64,
    pub message: String,
}

impl MailboxMessage {
    pub fn read(statement: &mut Statement) -> eyre::Result<Self> {
        Ok(MailboxMessage {
            init_peer_id: statement.read::<String>(0)?,
            timestamp: statement.read::<i64>(1)? as u64,
            message: statement.read::<String>(2)?,
        })
    }
}

#[marine]
/// `messages` contains up to `DEFAULT_MAX_MAILBOX` latest messages,
/// sorted in the order they were pushed
pub struct GetMailboxResult {
    pub messages: Vec<MailboxMessage>,
    pub success: bool,
    pub error: String,
}

impl From<eyre::Result<Vec<MailboxMessage>>> for GetMailboxResult {
    fn from(result: eyre::Result<Vec<MailboxMessage>>) -> Self {
        match result {
            Ok(messages) => GetMailboxResult {
                messages,
                success: true,
                error: "".to_string(),
            },
            Err(e) => GetMailboxResult {
                success: false,
                error: format!("get_mailbox error: {}", e),
                messages: vec![],
            },
        }
    }
}

#[marine]
// If there is no message, `message` is empty and `absent` is `true`.
pub struct PopMailboxResult {
    pub message: Vec<MailboxMessage>,
    pub success: bool,
    pub absent: bool,
    pub error: String,
}

impl From<eyre::Result<Option<MailboxMessage>>> for PopMailboxResult {
    fn from(value: eyre::Result<Option<MailboxMessage>>) -> Self {
        match value {
            Ok(Some(message)) => PopMailboxResult {
                message: vec![message],
                success: true,
                error: String::new(),
                absent: false,
            },
            Ok(None) => PopMailboxResult {
                message: vec![],
                success: true,
                error: String::new(),
                absent: true,
            },
            Err(e) => PopMailboxResult {
                message: vec![],
                success: false,
                error: format_error(e),
                absent: false,
            },
        }
    }
}

#[marine]
pub struct Log {
    pub timestamp: u64,
    pub message: String,
}

#[marine]
/// `logs` contains up to `DEFAULT_MAX_LOGS` latest logs with timestamps,
/// sorted in the order they were appeared
pub struct GetLogsResult {
    pub logs: Vec<Log>,
    pub success: bool,
    pub error: String,
}

impl From<eyre::Result<Vec<Log>>> for GetLogsResult {
    fn from(result: eyre::Result<Vec<Log>>) -> Self {
        match result {
            Ok(logs) => GetLogsResult {
                logs,
                success: true,
                error: "".to_string(),
            },
            Err(e) => GetLogsResult {
                success: false,
                error: format!("get_logs error: {}", e),
                logs: vec![],
            },
        }
    }
}
