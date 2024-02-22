use marine_rs_sdk::marine;

use fluence_spell_dtos::value::{GetLogsResult, Log, UnitValue};

use crate::auth::is_by_spell;
use crate::misc::fetch_rows;
use crate::schema::db;

#[marine]
/// Push a log to the db. It keeps `DEFAULT_MAX_LOGS` latest logs.
pub fn store_log(log: String) -> UnitValue {
    let call_parameters = marine_rs_sdk::get_call_parameters();

    // We want to prevent anyone except this spell to store logs to its kv
    if !is_by_spell(&call_parameters) {
        return UnitValue::error("store_log can be called only by the associated spell script");
    }

    let result: eyre::Result<()> = try {
        let conn = db();
        let mut statement = conn.prepare(r#"INSERT INTO logs (log) VALUES (?)"#)?;
        statement.bind(1, log.as_str())?;

        statement.next()?;
    };

    match result {
        Ok(_) => UnitValue::ok(),
        Err(e) => UnitValue::error(format!("store_log error: {}", e)),
    }
}

#[marine]
/// Get all logs ordered by timestamp ascending.
pub fn get_logs() -> GetLogsResult {
    let result: eyre::Result<Vec<Log>> = try {
        let conn = db();
        let statement =
            conn.prepare(r#"SELECT timestamp, log FROM logs ORDER BY timestamp ASC, id ASC"#)?;
        let logs: Vec<Log> = fetch_rows(statement, |statement| {
            Ok(Some(Log {
                timestamp: statement.read::<i64>(0)? as u64,
                message: statement.read::<String>(1)?,
            }))
        });

        logs
    };

    result.into()
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk::ParticleParameters;
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

    fn cp(service_id: String, particle_id: String) -> marine_rs_sdk_test::CallParameters {
        marine_rs_sdk_test::CallParameters {
            particle: ParticleParameters {
                init_peer_id: "folex".to_string(),
                id: particle_id,
                ..<_>::default()
            },
            service_creator_peer_id: "folex".to_string(),
            service_id,
            host_id: "".to_string(),
            worker_id: "".to_string(),
            tetraplets: vec![],
        }
    }

    #[marine_test(config_path = "../tests_artifacts/Config.toml")]
    fn test_store_log(spell: marine_test_env::spell::ModuleInterface) {
        println!("test_store_log started");

        let log1 = "logloglog1".to_string();
        let log2 = "logloglog2".to_string();
        let service_id = Uuid::new_v4();
        let particle_id = format!("spell_{}", service_id);
        let cp = cp(service_id.to_string(), particle_id);

        let store = spell.store_log_cp(log1.clone(), cp.clone());
        assert!(store.success, "{}", store.error);

        let store = spell.store_log_cp(log2.clone(), cp.clone());
        assert!(store.success, "{}", store.error);

        let logs = spell.get_logs();
        assert!(logs.success, "{}", logs.error);
        let logs = logs.logs;
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].message, log1);
        assert_eq!(logs[1].message, log2);
        assert!(logs[0].timestamp <= logs[1].timestamp);
    }

    #[marine_test(config_path = "../tests_artifacts/Config.toml")]
    fn test_store_log_fails_on_non_spell(spell: marine_test_env::spell::ModuleInterface) {
        let log = "logloglog".to_string();
        let service_id = Uuid::new_v4();
        let cp = cp(service_id.to_string(), "spell_WRONG_123".to_string());

        let logs_before: Vec<_> = spell.get_logs().logs;

        assert!(!spell.store_log_cp(log, cp).success);

        // make sure no log was inserted
        let logs_after: Vec<_> = spell.get_logs().logs;
        assert_eq!(logs_before.len(), logs_after.len());
    }

    #[marine_test(config_path = "../tests_artifacts/Config.toml")]
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
