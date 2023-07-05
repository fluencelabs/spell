use std::collections::HashMap;
use std::convert::TryFrom;

use eyre::WrapErr;
use marine_rs_sdk::marine;
use marine_sqlite_connector::{State, Statement};

use fluence_spell_dtos::value::UnitValue;

use crate::auth::{is_by_creator, is_by_spell};
use crate::schema::db;

/// The `%last_error%` content.
#[marine]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LastError {
    /// The error code.
    pub error_code: u32,
    /// The failed instruction.
    pub instruction: String,
    /// The error message.
    pub message: String,
    /// The peer where the call failed.
    pub peer_id: String,
}

#[marine]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LastErrorEntry {
    /// The reported error.
    pub last_error: LastError,
    /// The number of the error assigned in the script itself
    pub error_idx: u32,
}

impl TryFrom<&mut Statement> for LastErrorEntry {
    type Error = eyre::Error;

    /*
    particle_id
    timestamp
    error_idx
    error_code
    instruction
    message
    peer_id
    */
    fn try_from(statement: &mut Statement) -> Result<Self, Self::Error> {
        Ok(Self {
            error_idx: statement.read::<i64>(2)? as u32,
            last_error: LastError {
                error_code: statement
                    .read::<i64>(3)
                    .context("error reading error_code from row")?
                    as u32,
                instruction: statement
                    .read(4)
                    .context("error reading instruction from row")?,
                message: statement
                    .read(5)
                    .context("error reading message from row")?,
                peer_id: statement
                    .read(6)
                    .context("error reading peer_id from row")?,
            },
        })
    }
}

#[marine]
#[derive(Clone, Debug)]
pub struct ParticleErrors {
    pub particle_id: String,
    pub errors: Vec<LastErrorEntry>,
}

#[marine]
pub struct AllErrorsResult {
    pub particle_errors: Vec<ParticleErrors>,
    pub success: bool,
    pub error: String,
}

#[marine]
pub fn store_error(error: LastError, error_idx: u32, particle_timestamp: u64) -> UnitValue {
    let call_parameters = marine_rs_sdk::get_call_parameters();

    // We want to prevent anyone except this spell to store errors to its kv
    if !is_by_creator() || !is_by_spell(&call_parameters) {
        return UnitValue::error("store_error can be called only by the associated spell script");
    }

    let result: eyre::Result<()> = try {
        let conn = db();
        let mut statement = conn.prepare(
            r#"
        INSERT INTO errors
            (particle_id, timestamp, error_idx, error_code, instruction, message, peer_id)
        VALUES
            (?, ?, ?, ?, ?, ?, ?)
        "#,
        )?;
        statement.bind(1, call_parameters.particle_id.as_str())?;
        statement.bind(2, particle_timestamp as i64)?;
        statement.bind(3, error_idx as i64)?;
        statement.bind(4, error.error_code as i64)?;
        statement.bind(5, error.instruction.as_str())?;
        statement.bind(6, error.message.as_str())?;
        statement.bind(7, error.peer_id.as_str())?;

        statement.next()?;
    };

    match result {
        Ok(_) => UnitValue::ok(),
        Err(e) => UnitValue::error(format!("Error storing error: {}", e)),
    }
}

#[marine]
pub fn get_errors(particle_id: String) -> Vec<LastErrorEntry> {
    let result: eyre::Result<Vec<LastErrorEntry>> = try {
        let conn = db();
        let mut statement = conn.prepare(
            r#"
            SELECT
                *
            FROM
                errors WHERE particle_id = ?
        "#,
        )?;
        statement.bind(1, particle_id.as_str())?;
        std::iter::from_fn(move || {
            let r: eyre::Result<Option<LastErrorEntry>> = try {
                if let State::Row = statement.next()? {
                    Some(LastErrorEntry::try_from(&mut statement)?)
                } else {
                    None
                }
            };
            r.context("error fetching error row from sqlite")
                .transpose()
        })
        .filter_map(|r| r.ok())
        .collect()
    };

    result.unwrap_or_default()
}

#[marine]
pub fn get_all_errors() -> AllErrorsResult {
    let result: eyre::Result<Vec<ParticleErrors>> = try {
        let conn = db();
        let mut statement = conn.prepare(r#"SELECT * FROM errors"#)?;
        std::iter::from_fn(move || {
            let r: eyre::Result<Option<(String, LastErrorEntry)>> = try {
                if let State::Row = statement.next()? {
                    let particle_id = statement
                        .read::<String>(0)
                        .context("error reading particle_id from row")?;
                    let err = LastErrorEntry::try_from(&mut statement)
                        .context("error reading LastErrorEntry from row")?;
                    Some((particle_id, err))
                } else {
                    None
                }
            };
            r.context("error fetching error row from sqlite")
                .transpose()
        })
        .filter_map(|r| r.ok())
        .fold(HashMap::new(), |mut hm, (particle_id, error)| {
            hm.entry(particle_id).or_insert(Vec::new()).push(error);
            hm
        })
        .into_iter()
        .map(|(particle_id, errors)| ParticleErrors {
            particle_id,
            errors,
        })
        .collect()
    };

    match result {
        Ok(particle_errors) => AllErrorsResult {
            particle_errors,
            success: true,
            error: <_>::default(),
        },
        Err(err) => AllErrorsResult {
            particle_errors: <_>::default(),
            success: false,
            error: format!("{:?}", err),
        },
    }
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk_test::marine_test;
    use uuid::Uuid;

    use crate::schema::{DB_FILE, DEFAULT_MAX_ERR_PARTICLES};

    #[ctor::ctor]
    /// usage of 'ctor' makes this function run only once
    fn before_all_tests() {
        std::fs::remove_file(DB_FILE).ok();
    }

    /// after_each macro copy-pastes this function into every test
    fn after_each() {
        std::fs::remove_file(DB_FILE).ok();
    }

    fn cp(service_id: String, particle_id: String) -> CallParameters {
        CallParameters {
            init_peer_id: "folex".to_string(),
            service_creator_peer_id: "folex".to_string(),
            service_id,
            host_id: "".to_string(),
            particle_id,
            tetraplets: vec![],
        }
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_store_error(spell: marine_test_env::spell::ModuleInterface) {
        use marine_test_env::spell::LastError;

        println!("test_store_error started");

        let timestamp = 123;
        let error_idx = 321;
        let service_id = Uuid::new_v4();
        let particle_id = format!("spell_{}_123", service_id);
        let cp = cp(service_id.to_string(), particle_id.clone());
        let error = LastError {
            error_code: 1,
            instruction: "(null)".to_string(),
            message: "oh my god".to_string(),
            peer_id: "peerid".to_string(),
        };

        let store = spell.store_error_cp(error.clone(), error_idx, timestamp, cp);
        assert!(store.success, "{}", store.error);

        let errors = spell.get_all_errors();
        assert!(errors.success);
        let errors = errors.particle_errors;
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].particle_id, particle_id);
        assert_eq!(errors[0].errors.len(), 1);

        let e = &errors[0].errors[0];
        assert_eq!(e.error_idx, error_idx);
        assert_eq!(e.last_error.error_code, error.error_code);
        assert_eq!(e.last_error.instruction, error.instruction);
        assert_eq!(e.last_error.message, error.message);
        assert_eq!(e.last_error.peer_id, error.peer_id);
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_store_fails_on_non_spell(spell: marine_test_env::spell::ModuleInterface) {
        use marine_test_env::spell::LastError;

        let timestamp = 111;
        let error_idx = 2;
        let service_id = Uuid::new_v4();
        let cp = cp(service_id.to_string(), "spell_WRONG_123".to_string());

        let errors_before: Vec<_> = spell.get_all_errors().particle_errors;

        assert!(
            !spell
                .store_error_cp(
                    LastError {
                        error_code: 3,
                        instruction: "(null)".to_string(),
                        message: "oh my god".to_string(),
                        peer_id: "peerid".to_string(),
                    },
                    error_idx,
                    timestamp,
                    cp
                )
                .success
        );

        // make sure no error was inserted
        let errors_after: Vec<_> = spell.get_all_errors().particle_errors;
        assert_eq!(errors_before.len(), errors_after.len());
        for (before, after) in errors_before.into_iter().zip(errors_after) {
            assert_eq!(before.errors.len(), after.errors.len())
        }
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_error_lru(spell: marine_test_env::spell::ModuleInterface) {
        use marine_test_env::spell::LastError;

        let timestamp = 123;
        let error_idx = 321;
        let service_id = Uuid::new_v4();
        let error = LastError {
            error_code: 1,
            instruction: "(null)".to_string(),
            message: "oh my god".to_string(),
            peer_id: "peerid".to_string(),
        };

        for i in 0..(DEFAULT_MAX_ERR_PARTICLES + 5) {
            let particle_id = format!("spell_{}_{}", service_id, i);
            let cp = cp(service_id.to_string(), particle_id.clone());
            for _ in 0..2 {
                let store = spell.store_error_cp(error.clone(), error_idx, timestamp, cp.clone());
                assert!(store.success, "{} {}", i, store.error);
            }
        }

        let errors: Vec<_> = spell.get_all_errors().particle_errors;
        assert_eq!(errors.len(), DEFAULT_MAX_ERR_PARTICLES);
        for err in errors {
            assert_eq!(err.errors.len(), 2);
        }
    }
}
