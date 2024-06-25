/*
 * Copyright 2024 Fluence DAO
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
