use marine_sqlite_connector::Error as SqliteError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum SpellError {
    #[error("Internal Sqlite error: {0}")]
    SqliteError(
        #[from]
        #[source]
        SqliteError,
    ),
    #[error("Key '{0}' does not exist")]
    KeyNotExists(String),
    #[error("Location not available: relay was not set")]
    NoRelay,
    #[error("Only owner of the spell can set relay peer id")]
    SetRelayForbidden,
    #[error("Relay was already set and cannot be changed")]
    RelayAlreadySet,
    #[error("Only owner of the spell can set trigger config")]
    SetTriggerConfigForbidden,
    #[error("Trigger Config is not set. Use set_trigger_config to set it.")]
    NoTriggerConfig,
}
