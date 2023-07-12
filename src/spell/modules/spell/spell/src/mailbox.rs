use marine_rs_sdk::marine;

use fluence_spell_dtos::value::{StringValue, UnitValue};

use crate::auth::is_by_spell;
use crate::kv::primitive::read_string;
use crate::misc::fetch_rows;
use crate::schema::db;

#[marine]
/// `messages` contains up to `DEFAULT_MAX_MAILBOX` latest messages,
/// sorted in the order they were pushed
pub struct GetMailboxResult {
    pub messages: Vec<String>,
    pub success: bool,
    pub error: String,
}

impl From<eyre::Result<Vec<String>>> for GetMailboxResult {
    fn from(result: eyre::Result<Vec<String>>) -> Self {
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
/// Push a message to the mailbox. Mailbox keeps `DEFAULT_MAX_MAILBOX` latest messages.
pub fn push_mailbox(message: String) -> UnitValue {
    let result: eyre::Result<()> = try {
        let conn = db();
        let mut statement = conn.prepare(r#" INSERT INTO mailbox (message) VALUES (?)"#)?;
        statement.bind(1, message.as_str())?;
        statement.next()?;
    };

    match result {
        Ok(_) => UnitValue::ok(),
        Err(e) => UnitValue::error(format!("Error storing mailbox message: {}", e)),
    }
}

#[marine]
/// Get all messages from the mailbox ordered by timestamp ascending.
pub fn get_mailbox() -> GetMailboxResult {
    let result: eyre::Result<Vec<String>> = try {
        let conn = db();
        let statement =
            conn.prepare(r#"SELECT message FROM mailbox ORDER BY timestamp ASC, id ASC"#)?;
        let messages: Vec<String> = fetch_rows(statement, |statement| {
            Ok(Some(statement.read::<String>(0)?))
        });

        messages
    };

    result.into()
}

#[marine]
/// Get the latest mailbox message and remove it from the mailbox.
/// result.absent is true if there are no messages in the mailbox.
pub fn pop_mailbox() -> StringValue {
    let call_parameters = marine_rs_sdk::get_call_parameters();

    // We want to prevent anyone except this spell to pop from mailbox
    if !is_by_spell(&call_parameters) {
        return Err(eyre::eyre!(
            "pop_mailbox can be called only by the associated spell script"
        ))
        .into();
    }

    let db = db();
    let result: eyre::Result<Option<String>> = try {
        let mut get = db.prepare(
            r#" SELECT message, id FROM mailbox ORDER BY timestamp DESC, id DESC LIMIT 1"#,
        )?;
        let string = read_string(&mut get, 0)?;
        let id = get.read::<i64>(1)?;

        let mut delete = db.prepare(r#"DELETE FROM mailbox WHERE id = ?"#)?;
        delete.bind(1, id)?;
        delete.next()?;

        string
    };

    result.into()
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
    fn test_push_mailbox(spell: marine_test_env::spell::ModuleInterface) {
        println!("test_push_mailbox started");

        let message1 = "message1".to_string();
        let message2 = "message2".to_string();
        let service_id = Uuid::new_v4();
        let particle_id = format!("spell_{}", service_id);
        let cp = cp(service_id.to_string(), particle_id);

        let store = spell.push_mailbox_cp(message1.clone(), cp.clone());
        assert!(store.success, "{}", store.error);

        let store = spell.push_mailbox_cp(message2.clone(), cp);
        assert!(store.success, "{}", store.error);

        let messages = spell.get_mailbox();
        assert!(messages.success, "{}", messages.error);
        let messages = messages.messages;
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], message1);
        assert_eq!(messages[1], message2);
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_pop_mailbox_fails_on_non_spell(spell: marine_test_env::spell::ModuleInterface) {
        let message = "message".to_string();
        let service_id = Uuid::new_v4();
        let particle_id = format!("spell_{}", service_id);
        let cp = cp(service_id.to_string(), particle_id);

        let store = spell.push_mailbox_cp(message.clone(), cp);
        assert!(store.success, "{}", store.error);
        let messages_before: Vec<_> = spell.get_mailbox().messages;

        let pop = spell.pop_mailbox();
        assert!(!pop.success);

        // make sure no message was popped
        let messages_after: Vec<_> = spell.get_mailbox().messages;
        assert_eq!(messages_before.len(), messages_after.len());
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_mailbox_lru(spell: marine_test_env::spell::ModuleInterface) {
        let message = "message".to_string();
        let service_id = Uuid::new_v4();

        for i in 0..(DEFAULT_MAX_MAILBOX + 5) {
            let particle_id = format!("spell_{}_{}", service_id, i);
            let cp = cp(service_id.to_string(), particle_id.clone());
            let store = spell.push_mailbox_cp(message.clone(), cp.clone());
            assert!(store.success, "{} {}", i, store.error);
        }

        let messages: Vec<_> = spell.get_mailbox().messages;
        assert_eq!(messages.len(), DEFAULT_MAX_MAILBOX);
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_pop_mailbox(spell: marine_test_env::spell::ModuleInterface) {
        let message = "message".to_string();
        let service_id = Uuid::new_v4();
        let particle_id = format!("spell_{}", service_id);
        let cp = cp(service_id.to_string(), particle_id);

        let store = spell.push_mailbox_cp(message.clone(), cp.clone());
        assert!(store.success, "{}", store.error);

        let messages = spell.get_mailbox();
        assert!(messages.success, "{}", messages.error);
        let messages = messages.messages;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], message);

        let pop = spell.pop_mailbox_cp(cp);
        assert!(pop.success, "{}", pop.error);
        assert_eq!(pop.str, message);

        let messages = spell.get_mailbox();
        assert!(messages.success, "{}", messages.error);
        assert_eq!(messages.messages.len(), 0);
    }
}
