use thiserror::Error as ThisError;
use marine_sqlite_connector::Error as SqliteError;

#[derive(ThisError, Debug)]
pub enum SpellError {
    #[error("Internal Sqlite error: {0}")]
    SqliteError(
        #[from]
        #[source]
        SqliteError,
    ),
    #[error("Key {0} does not exist")]
    KeyNotExists(String),
}