/*
 * Aqua Spell Service
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use marine_rs_sdk::marine;
use marine_sqlite_connector::State;

use fluence_spell_dtos::value::{GetMailboxResult, MailboxMessage, PopMailboxResult, UnitValue};

use crate::auth::is_by_spell;
use crate::misc::fetch_rows;
use crate::schema::db;

#[marine]
/// Push a message to the mailbox. Mailbox keeps `DEFAULT_MAX_MAILBOX` latest messages.
pub fn push_mailbox(message: String) -> UnitValue {
    let init_peer_id = marine_rs_sdk::get_call_parameters().particle.init_peer_id;
    let result: eyre::Result<()> = try {
        let conn = db();
        let mut statement =
            conn.prepare(r#" INSERT INTO mailbox (init_peer_id, message) VALUES (?, ?)"#)?;
        statement.bind(1, init_peer_id.as_str())?;
        statement.bind(2, message.as_str())?;
        statement.next()?;
    };

    match result {
        Ok(_) => UnitValue::ok(),
        Err(e) => UnitValue::error(format!("Error storing mailbox message: {}", e)),
    }
}

#[marine]
/// Get all messages from the mailbox in FIFO order.
pub fn get_mailbox() -> GetMailboxResult {
    let result: eyre::Result<Vec<MailboxMessage>> = try {
        let conn = db();
        let statement = conn
            .prepare(r#"SELECT init_peer_id, timestamp, message FROM mailbox ORDER BY id DESC"#)?;
        let messages: Vec<MailboxMessage> = fetch_rows(statement, |statement| {
            Ok(Some(MailboxMessage::read(statement)?))
        });

        messages
    };

    result.into()
}

#[marine]
/// Get the latest mailbox message and remove it from the mailbox.
/// result.absent is true if there are no messages in the mailbox.
pub fn pop_mailbox() -> PopMailboxResult {
    let call_parameters = marine_rs_sdk::get_call_parameters();

    // We want to prevent anyone except this spell to pop from mailbox
    if !is_by_spell(&call_parameters) {
        return Err(eyre::eyre!(
            "pop_mailbox can be called only by the associated spell script"
        ))
        .into();
    }

    let db = db();
    let result: eyre::Result<Option<MailboxMessage>> = try {
        let mut get = db.prepare(
            r#" SELECT init_peer_id, timestamp, message, id FROM mailbox ORDER BY id DESC LIMIT 1"#,
        )?;

        let mut message = None;
        if let State::Row = get.next()? {
            message = Some(MailboxMessage::read(&mut get)?);

            let id = get.read::<i64>(3)?;
            let mut delete = db.prepare(r#"DELETE FROM mailbox WHERE id = ?"#)?;
            delete.bind(1, id)?;
            delete.next()?;
        }

        message
    };

    result.into()
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk::ParticleParameters;
    use marine_rs_sdk_test::marine_test;
    use uuid::Uuid;

    use crate::schema::DEFAULT_MAX_MAILBOX;

    const DB_FILE: &str = "./tests_artifacts/spell.sqlite";

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
    fn test_push_mailbox(spell: marine_test_env::spell::ModuleInterface) {
        println!("test_push_mailbox started");

        let message1 = "message1".to_string();
        let message2 = "message2".to_string();
        let service_id = Uuid::new_v4();
        let particle_id = format!("spell_{}", service_id);
        let cp = cp(service_id.to_string(), particle_id);

        let store = spell.push_mailbox_cp(message1.clone(), cp.clone());
        assert!(store.success, "{}", store.error);

        let store = spell.push_mailbox_cp(message2.clone(), cp.clone());
        assert!(store.success, "{}", store.error);

        let messages = spell.get_mailbox();
        assert!(messages.success, "{}", messages.error);
        let messages = messages.messages;
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].message, message2);
        assert_eq!(messages[0].init_peer_id, cp.particle.init_peer_id);
        assert_eq!(messages[1].message, message1);
        assert_eq!(messages[1].init_peer_id, cp.particle.init_peer_id);
        assert!(messages[0].timestamp >= messages[1].timestamp);
    }

    #[marine_test(config_path = "../tests_artifacts/Config.toml")]
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

    #[marine_test(config_path = "../tests_artifacts/Config.toml")]
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

    #[marine_test(config_path = "../tests_artifacts/Config.toml")]
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
        assert_eq!(messages[0].message, message);

        let pop = spell.pop_mailbox_cp(cp);
        assert!(pop.success, "{}", pop.error);
        assert!(!pop.absent);
        assert_eq!(pop.message.len(), 1);
        assert_eq!(pop.message[0].message, message);

        let messages = spell.get_mailbox();
        assert!(messages.success, "{}", messages.error);
        assert_eq!(messages.messages.len(), 0);
    }
}
