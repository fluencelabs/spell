use eyre::WrapErr;
use marine_rs_sdk::marine;
use marine_sqlite_connector::State;

use fluence_spell_dtos::value::UnitValue;

use crate::auth::{is_by_creator, is_by_spell};
use crate::schema::db;


#[marine]
pub struct AllMailboxResult {
    pub messages: Vec<String>,
    pub success: bool,
    pub error: String,
}

#[marine]
pub fn store_mailbox(message: String, timestamp: u64) -> UnitValue {
    let call_parameters = marine_rs_sdk::get_call_parameters();

    // We want to prevent anyone except this spell to store logs to its kv
    if !is_by_creator() || !is_by_spell(&call_parameters) {
        return UnitValue::error("store_mailbox can be called only by the associated spell script");
    }

    let result: eyre::Result<()> = try {
        let conn = db();
        let mut statement = conn.prepare(
            r#"
        INSERT INTO mailbox
            (timestamp, message)
        VALUES
            (?, ?)
        "#,
        )?;
        statement.bind(1, timestamp as i64)?;
        statement.bind(2, message.as_str())?;


        statement.next()?;
    };

    match result {
        Ok(_) => UnitValue::ok(),
        Err(e) => UnitValue::error(format!("Error storing mailbox message: {}", e)),
    }
}

#[marine]
pub fn get_mailbox() -> AllMailboxResult {
    let result: eyre::Result<Vec<String>> = try {
        let conn = db();
        let mut statement = conn.prepare(
            r#"
            SELECT
                message
            FROM
                mailbox
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
            r.context("error fetching mailbox message row from sqlite")
                .transpose()
        })
            .filter_map(|r| r.ok())
            .collect()
    };

    match result {
        Ok(messages) => AllMailboxResult {
            messages,
            success: true,
            error: "".to_string(),
        },
        Err(e) => {
            AllMailboxResult {
                success: false,
                error: format!("Error getting all logs: {}", e),
                messages: vec![],
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

    use crate::schema::{DB_FILE, DEFAULT_MAX_MAILBOX};

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
    fn test_store_mailbox(spell: marine_test_env::spell::ModuleInterface) {
        println!("test_store_mailbox started");

        let timestamp = 123;
        let message = "message".to_string();
        let service_id = Uuid::new_v4();
        let particle_id = format!("spell_{}", service_id);
        let cp = cp(service_id.to_string(), particle_id);


        let store = spell.store_mailbox_cp(message.clone(), timestamp, cp);
        assert!(store.success, "{}", store.error);

        let messages = spell.get_mailbox();
        assert!(messages.success, "{}", messages.error);
        let messages = messages.messages;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], message);
    }

    #[marine_test(
    config_path = "../tests_artifacts/Config.toml",
    modules_dir = "../tests_artifacts"
    )]
    fn test_store_mailbox_fails_on_non_spell(spell: marine_test_env::spell::ModuleInterface) {
        let timestamp = 111;
        let message = "message".to_string();
        let service_id = Uuid::new_v4();
        let cp = cp(service_id.to_string(), "spell_WRONG_123".to_string());

        let messages_before: Vec<_> = spell.get_mailbox().messages;

        assert!(
            !spell
                .store_mailbox_cp(
                    message,
                    timestamp,
                    cp
                )
                .success
        );

        // make sure no message was inserted
        let messages_after: Vec<_> = spell.get_mailbox().messages;
        assert_eq!(messages_before.len(), messages_after.len());
    }

    #[marine_test(
    config_path = "../tests_artifacts/Config.toml",
    modules_dir = "../tests_artifacts"
    )]
    fn test_mailbox_lru(spell: marine_test_env::spell::ModuleInterface) {
        let timestamp = 123;
        let message = "message".to_string();
        let service_id = Uuid::new_v4();

        for i in 0..(DEFAULT_MAX_MAILBOX + 5) {
            let particle_id = format!("spell_{}_{}", service_id, i);
            let cp = cp(service_id.to_string(), particle_id.clone());
            let store = spell.store_mailbox_cp(message.clone(), timestamp, cp.clone());
            assert!(store.success, "{} {}", i, store.error);
        }

        let messages: Vec<_> = spell.get_mailbox().messages;
        assert_eq!(messages.len(), DEFAULT_MAX_MAILBOX);
    }
}
