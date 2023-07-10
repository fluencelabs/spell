use eyre::WrapErr;
use marine_rs_sdk::marine;
use marine_sqlite_connector::State;

use fluence_spell_dtos::value::UnitValue;

use crate::auth::{is_by_creator, is_by_spell};
use crate::schema::db;


#[marine]
pub struct AllLogsResult {
    pub logs: Vec<String>,
    pub success: bool,
    pub error: String,
}

#[marine]
pub fn store_log(log: String) -> UnitValue {
    let call_parameters = marine_rs_sdk::get_call_parameters();

    // We want to prevent anyone except this spell to store logs to its kv
    if !is_by_creator() || !is_by_spell(&call_parameters) {
        return UnitValue::error("store_log can be called only by the associated spell script");
    }

    let result: eyre::Result<()> = try {
        let conn = db();
        let mut statement = conn.prepare(
            r#"
        INSERT INTO logs
            (log)
        VALUES
            (?)
        "#,
        )?;
        statement.bind(1, log.as_str())?;


        statement.next()?;
    };

    match result {
        Ok(_) => UnitValue::ok(),
        Err(e) => UnitValue::error(format!("Error storing log: {}", e)),
    }
}

#[marine]
pub fn get_logs() -> AllLogsResult {
    let result: eyre::Result<Vec<String>> = try {
        let conn = db();
        let mut statement = conn.prepare(
            r#"
            SELECT
                log
            FROM
                logs
        "#,
        )?;
        std::iter::from_fn(move || {
            let r: eyre::Result<Option<String>> = try {
                if let State::Row = statement.next()? {
                    Some(statement.read::<String>(0)?)
                } else {
                    None
                }
            };
            r.context("error fetching log row from sqlite")
                .transpose()
        })
            .filter_map(|r| r.ok())
            .collect()
    };

    match result {
        Ok(logs) => AllLogsResult {
            logs,
            success: true,
            error: "".to_string(),
        },
        Err(e) => {
            AllLogsResult {
                success: false,
                error: format!("Error getting all logs: {}", e),
                logs: vec![],
            }
        }
    }
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk_test::marine_test;
    use uuid::Uuid;

    use crate::schema::{DB_FILE, DEFAULT_MAX_LOGS};

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
            particle_id,
            service_id,
            host_id: "".to_string(),
            tetraplets: vec![],
        }
    }

    #[marine_test(
    config_path = "../tests_artifacts/Config.toml",
    modules_dir = "../tests_artifacts"
    )]
    fn test_store_log(spell: marine_test_env::spell::ModuleInterface) {
        println!("test_store_log started");

        let log = "logloglog".to_string();
        let service_id = Uuid::new_v4();
        let particle_id = format!("spell_{}", service_id);
        let cp = cp(service_id.to_string(), particle_id);


        let store = spell.store_log_cp(log.clone(), cp);
        assert!(store.success, "{}", store.error);

        let logs = spell.get_logs();
        assert!(logs.success, "{}", logs.error);
        let logs = logs.logs;
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], log);
    }

    #[marine_test(
    config_path = "../tests_artifacts/Config.toml",
    modules_dir = "../tests_artifacts"
    )]
    fn test_store_log_fails_on_non_spell(spell: marine_test_env::spell::ModuleInterface) {
        let log = "logloglog".to_string();
        let service_id = Uuid::new_v4();
        let cp = cp(service_id.to_string(), "spell_WRONG_123".to_string());

        let logs_before: Vec<_> = spell.get_logs().logs;

        assert!(
            !spell
                .store_log_cp(
                    log,
                    cp
                )
                .success
        );

        // make sure no log was inserted
        let logs_after: Vec<_> = spell.get_logs().logs;
        assert_eq!(logs_before.len(), logs_after.len());
    }

    #[marine_test(
    config_path = "../tests_artifacts/Config.toml",
    modules_dir = "../tests_artifacts"
    )]
    fn test_log_lru(spell: marine_test_env::spell::ModuleInterface) {
        let log = "logloglog".to_string();
        let service_id = Uuid::new_v4();

        for i in 0..(DEFAULT_MAX_LOGS + 5) {
            let particle_id = format!("spell_{}_{}", service_id, i);
            let cp = cp(service_id.to_string(), particle_id.clone());
            let store = spell.store_log_cp(log.clone(), cp.clone());
            assert!(store.success, "{} {}", i, store.error);
        }

        let logs: Vec<_> = spell.get_logs().logs;
        assert_eq!(logs.len(), DEFAULT_MAX_LOGS);
    }
}
