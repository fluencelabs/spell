use fluence_spell_dtos::value::{StringListValue, StringValue, UnitValue};
use marine_rs_sdk::marine;
use marine_sqlite_connector::State;
use crate::auth::guard_kv_write;

use crate::misc::fetch_rows;
use crate::schema::db;

#[marine]
pub fn list_push_string(key: &str, value: String) -> UnitValue {
    let result: eyre::Result<()> = try {
        guard_kv_write(key)?;
        let conn = db();
        let mut statement = conn.prepare(
            r#"
                INSERT INTO kv (key, string, list_order)
                    VALUES (
                        ?,
                        ?,
                        COALESCE(
                            (
                                SELECT MAX(list_order) + 1
                                FROM kv
                                WHERE key = ?
                            ),
                            0
                        )
                    )
            "#,
        )?;
        statement.bind(1, key)?;
        statement.bind(2, value.as_str())?;
        statement.bind(3, key)?;
        statement.next()?;
    };

    result.into()
}

#[marine]
/// Remove latest element in a list of strings, and return it
pub fn list_pop_string(key: &str) -> StringValue {
    let result: eyre::Result<Option<String>> = try {
        guard_kv_write(key)?;
        let db = db();
        let mut get = db.prepare(
            r#"
            SELECT string, max(list_order) FROM kv
                WHERE key = ?
                  AND list_order >= 0
        "#,
        )?;
        get.bind(1, key)?;

        let mut result = None;
        if let State::Row = get.next()? {
            let val = get.read::<String>(0)?;
            if let Some(list_order) = get.read::<Option<i64>>(1)? {
                let mut delete = db.prepare(r#"DELETE FROM kv WHERE key = ? AND list_order = ?"#)?;
                delete.bind(1, key)?;
                delete.bind(2, list_order)?;
                delete.next()?;

                result = Some(val.to_string());
            }
        };

        result
    };

    result.into()
}

#[marine]
/// Get a whole list of strings
pub fn list_get_strings(key: &str) -> StringListValue {
    let conn = db();
    let result: eyre::Result<Vec<String>> = try {
        let mut statement = conn.prepare(
            r#"
            SELECT
                string
            FROM
                kv
           WHERE key = ?
             AND list_order >= 0
        ORDER BY
                list_order ASC
        "#,
        )?;
        statement.bind(1, key)?;
        let list: Vec<String> = fetch_rows(statement, |statement| {
            let val = statement.read::<String>(0)?;
            Ok(Some(val.to_string()))
        });

        list
    };

    result.into()
}

#[marine]
/// Remove a value from a list of strings. If the value is in several places, remove all of them.
/// Returns an error on exceptions. Doesn't report if the value was actually removed.
pub fn list_remove_string(key: &str, value: &str) -> UnitValue {
    let result: eyre::Result<()> = try {
        guard_kv_write(key)?;
        let conn = db();
        // list_order == -1 when the key isn't a part of a list
        let mut statement = conn.prepare(
            r#"
            DELETE FROM kv
                WHERE key = ?
                  AND (string = ? OR (string IS NULL AND ? IS NULL))
                  AND list_order != -1
        "#,
        )?;
        statement.bind(1, key)?;
        statement.bind(2, value)?;
        statement.bind(3, value)?;
        statement.next()?;
    };

    result.into()
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk_test::marine_test;

    use crate::schema::DB_FILE;

    #[ctor::ctor]
    /// usage of 'ctor' makes this function run only once
    fn before_all_tests() {
        std::fs::remove_file(DB_FILE).ok();
    }

    /// after_each macro copy-pastes this function into every test
    fn after_each() {
        std::fs::remove_file(DB_FILE).ok();
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_push_get(spell: marine_test_env::spell::ModuleInterface) {
        let key = "a";

        let push = spell.list_push_string_cp(key.into(), "b".into(), spell_call_params());
        assert!(push.success, "1 push failed {}", push.error);
        let push = spell.list_push_string_cp(key.into(), "f".into(), spell_call_params());
        assert!(push.success, "2 push failed {}", push.error);
        let push = spell.list_push_string_cp(key.into(), "в".into(), spell_call_params());
        assert!(push.success, "3 push failed {}", push.error);

        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.value, vec!["b", "f", "в"]);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_push_pop_get(spell: marine_test_env::spell::ModuleInterface) {
        type SPELL = marine_test_env::spell::ModuleInterface;
        let key = "a";

        let check_len = |spell: &mut SPELL, len| {
            let get = spell.list_get_strings(key.into());
            assert!(get.success, "list_get_strings failed {}", get.error);
            assert_eq!(
                get.value.len(),
                len,
                "expected {}, got {}",
                len,
                get.value.len()
            );
        };

        let pop = |spell: &mut SPELL, expectation| {
            let pop = spell.list_pop_string_cp(key.into(), spell_call_params());
            assert!(pop.success, "pop failed {}", pop.error);
            assert_eq!(pop.value, expectation);
        };

        let push = |spell: &mut SPELL, value: &str| {
            let push = spell.list_push_string_cp(key.into(), value.into(), spell_call_params());
            assert!(push.success, "1 push failed {}", push.error);
        };

        check_len(&mut spell, 0);
        push(&mut spell, "b");
        check_len(&mut spell, 1);
        pop(&mut spell, "b");
        check_len(&mut spell, 0);
        push(&mut spell, "хаха");
        push(&mut spell, "хаха");
        push(&mut spell, "haha");
        check_len(&mut spell, 3);

        let get = spell.list_get_strings(key.into());
        assert_eq!(get.value, &["хаха", "хаха", "haha"]);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_list_remove(spell: marine_test_env::spell::ModuleInterface) {
        let key = "a";
        let value = "value_to_remove";

        type SPELL = marine_test_env::spell::ModuleInterface;

        let push = |spell: &mut SPELL, value: &str| {
            let _ = spell.list_push_string_cp(key.into(), value.into(), spell_call_params());
        };
        let remove = |spell: &mut SPELL, key: &str, value: &str| {
            spell.list_remove_string_cp(key.into(), value.into(), spell_call_params())
        };

        push(&mut spell, "b");
        push(&mut spell, value);
        push(&mut spell, "в");
        push(&mut spell, "p");
        push(&mut spell, value);
        push(&mut spell, "a");

        let removed = spell.list_remove_string_cp(key.into(), value.into(), spell_call_params());
        assert!(removed.success, "remove failed {}", removed.error);

        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.value, vec!["b", "в", "p", "a"]);

        let removed = remove(&mut spell, key, "not-in-list");
        assert!(removed.success, "remove of non-existent values from the list must return ok: {}", removed.error);

        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.value, vec!["b", "в", "p", "a"], "must be the same after removing of absent value");

        let _ = spell.set_string_cp("str".into(), "val".into(), spell_call_params());
        let removed = remove(&mut spell, "str", "val");
        assert!(removed.success, "remove of non-existent key-values from a list must return ok: {}", removed.error);

        let get = spell.get_string("str".into());
        assert!(!get.absent, "non-list value must not be removed");
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_list_remove_and_add(spell: marine_test_env::spell::ModuleInterface) {
        let key = "a";
        let value = "value_to_remove";
        let value_first = "value_first";

        type SPELL = marine_test_env::spell::ModuleInterface;

        let push = |spell: &mut SPELL, value: &str| {
            let _ = spell.list_push_string_cp(key.into(), value.into(), spell_call_params());
        };
        let remove = |spell: &mut SPELL, key: &str, value: &str| {
            spell.list_remove_string_cp(key.into(), value.into(), spell_call_params())
        };

        push(&mut spell, value_first);
        push(&mut spell, value);
        push(&mut spell, "в");
        push(&mut spell, "p");
        push(&mut spell, value);
        push(&mut spell, "a");
        let _ = remove(&mut spell, key, value);
        push(&mut spell, "n1");
        push(&mut spell, "n2");

        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.value, vec![value_first, "в", "p", "a", "n1", "n2"]);

        let _ = remove(&mut spell, key, value_first);
        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.value, vec!["в", "p", "a", "n1", "n2"]);

    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_push_pop_same_keys(spell: marine_test_env::spell::ModuleInterface) {
        let key = "a";
        let val1 = "a";
        let val2 = "b";

        let _ = spell.set_string_cp(key.into(), val1.into(), spell_call_params());
        let _ = spell.list_push_string_cp(key.into(), val2.into(), spell_call_params());
        let list = spell.list_get_strings(key.into());
        assert!(list.success, "list_get_strings failed {}", list.error);
        assert_eq!(list.value, vec![val2], "list must contain only the elements we put in it");

        let key = "b";
        let val1 = "a";

        let _ = spell.set_string_cp(key.into(), val1.into(), spell_call_params());
        let pop = spell.list_pop_string_cp(key.into(), spell_call_params());
        assert!(pop.success, "pop failed {}", pop.error);
        assert!(pop.absent, "pop mustn't return string value");

    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_push_pop_empty_string(spell: marine_test_env::spell::ModuleInterface) {
        let key = "a";
        let empty = "";
        let val1 = "a";
        let val2 = "b";
        type SPELL = marine_test_env::spell::ModuleInterface;

        let push = |spell: &mut SPELL, value: &str| {
            let _ = spell.list_push_string_cp(key.into(), value.into(), spell_call_params());
        };

        let _ = push(&mut spell, val1);
        let _ = push(&mut spell, empty);
        let _ = push(&mut spell, val2);

        let list = spell.list_get_strings(key.into());
        assert!(list.success, "list_get_strings failed {}", list.error);
        assert_eq!(list.value, vec![val1, empty, val2]);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_remove_empty_string(spell: marine_test_env::spell::ModuleInterface) {
        let key = "a";
        let empty = "";
        let val1 = "a";
        let val2 = "b";
        type SPELL = marine_test_env::spell::ModuleInterface;

        let push = |spell: &mut SPELL, value: &str| {
            let _ = spell.list_push_string_cp(key.into(), value.into(), spell_call_params());
        };

        push(&mut spell, val1);
        push(&mut spell, empty);
        push(&mut spell, empty);
        push(&mut spell, empty);
        push(&mut spell, empty);
        push(&mut spell, val2);

        let _ = spell.list_remove_string_cp(key.into(), empty.into(), spell_call_params());
        let list = spell.list_get_strings(key.into());
        assert!(list.success, "list_get_strings failed {}", list.error);
        assert_eq!(list.value, vec![val1, val2]);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_list_host(spell: marine_test_env::spell::ModuleInterface) {
        let private_key = "key";
        let host_key = "h_key";
        let host_worker_key = "hw_key";
        let worker_key = "w_key";
        let value = "value";

        type SPELL = marine_test_env::spell::ModuleInterface;
        let cp = || host_call_params();
        let push_ok = |spell: &mut SPELL, key: &str, value: &str| {
            let pushed = spell.list_push_string_cp(key.into(), value.into(), cp());
            assert!(pushed.success, "pushing ({}, {})", key, value);
            let list = spell.list_get_strings(key.into());
            assert!(list.value.contains(&value.to_string()));
        };
        let push_failed = |spell: &mut SPELL, key: &str, value: &str| {
            let list_before = spell.list_get_strings(key.into());
            let pushed = spell.list_push_string_cp(key.into(), value.into(), cp());
            assert!(!pushed.success, "pushing ({}, {})", key, value);
            let list_after = spell.list_get_strings(key.into());
            assert_eq!(list_before.value.len(), list_after.value.len());
        };
        let pop_ok = |spell: &mut SPELL, key: &str| {
            let popped = spell.list_pop_string_cp(key.into(), cp());
            assert!(popped.success, "popping {}", key);
            let list = spell.list_get_strings(key.into());
            assert!(list.value.is_empty());
        };
        let pop_failed = |spell: &mut SPELL, key: &str| {
            let popped = spell.list_pop_string_cp(key.into(), cp());
            assert!(!popped.success, "popping {}", key);
            let list = spell.list_get_strings(key.into());
            assert!(!list.value.is_empty());
        };
        let remove_ok = |spell: &mut SPELL, key: &str| {
            let removed = spell.list_remove_string_cp(key.into(), value.into(), cp());
            assert!(removed.success, "removing ({}, {})", key, value);
            let list = spell.list_get_strings(key.into());
            assert!(list.value.is_empty());
        };
        let remove_failed = |spell: &mut SPELL, key: &str| {
            let removed = spell.list_remove_string_cp(key.into(), value.into(), cp());
            assert!(!removed.success, "removing ({}, {})", key, value);
            let list = spell.list_get_strings(key.into());
            assert!(!list.value.is_empty());
        };

        // check push
        push_failed(&mut spell, private_key, value);
        push_failed(&mut spell, worker_key, value);
        push_ok(&mut spell, host_worker_key, value);
        push_ok(&mut spell, host_key, value);

        // chek pop
        spell.list_push_string_cp(private_key.into(), value.into(), spell_call_params());
        spell.list_push_string_cp(worker_key.into(), value.into(), spell_call_params());

        pop_failed(&mut spell, private_key);
        pop_failed(&mut spell, worker_key);
        pop_ok(&mut spell, host_worker_key);
        pop_ok(&mut spell, host_key);

        // check remove
        push_ok(&mut spell, host_worker_key, value);
        push_ok(&mut spell, host_key, value);

        remove_failed(&mut spell, private_key);
        remove_failed(&mut spell, worker_key);
        remove_ok(&mut spell, host_worker_key);
        remove_ok(&mut spell, host_key);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_list_worker(spell: marine_test_env::spell::ModuleInterface) {
        let private_key = "key";
        let host_key = "h_key";
        let host_worker_key = "hw_key";
        let worker_key = "w_key";
        let value = "value";

        let spell = &mut spell;
        type SPELL = marine_test_env::spell::ModuleInterface;
        let cp = || worker_call_params();
        let push_ok = |spell: &mut SPELL, key: &str, value: &str| {
            let pushed = spell.list_push_string_cp(key.into(), value.into(), cp());
            assert!(pushed.success, "pushing ({}, {})", key, value);
            let list = spell.list_get_strings(key.into());
            assert!(list.value.contains(&value.to_string()));
        };
        let push_failed = |spell: &mut SPELL, key: &str, value: &str| {
            let list_before = spell.list_get_strings(key.into());
            let pushed = spell.list_push_string_cp(key.into(), value.into(), cp());
            assert!(!pushed.success, "pushing ({}, {})", key, value);
            let list_after = spell.list_get_strings(key.into());
            assert_eq!(list_before.value.len(), list_after.value.len());
        };
        let pop_ok = |spell: &mut SPELL, key: &str| {
            let popped = spell.list_pop_string_cp(key.into(), cp());
            assert!(popped.success, "popping {}", key);
            let list = spell.list_get_strings(key.into());
            assert!(list.value.is_empty());
        };
        let pop_failed = |spell: &mut SPELL, key: &str| {
            let popped = spell.list_pop_string_cp(key.into(), cp());
            assert!(!popped.success, "popping {}", key);
            let list = spell.list_get_strings(key.into());
            assert!(!list.value.is_empty());
        };
        let remove_ok = |spell: &mut SPELL, key: &str| {
            let removed = spell.list_remove_string_cp(key.into(), value.into(), cp());
            assert!(removed.success, "removing ({}, {})", key, value);
            let list = spell.list_get_strings(key.into());
            assert!(list.value.is_empty());
        };
        let remove_failed = |spell: &mut SPELL, key: &str| {
            let removed = spell.list_remove_string_cp(key.into(), value.into(), cp());
            assert!(!removed.success, "removing ({}, {})", key, value);
            let list = spell.list_get_strings(key.into());
            assert!(!list.value.is_empty());
        };

        // check push
        push_failed(spell, private_key, value);
        push_ok(spell, worker_key, value);
        push_ok(spell, host_worker_key, value);
        push_failed(spell, host_key, value);

        // chek pop
        spell.list_push_string_cp(private_key.into(), value.into(), spell_call_params());
        spell.list_push_string_cp(host_key.into(), value.into(), spell_call_params());

        pop_failed(spell, private_key);
        pop_ok(spell, worker_key);
        pop_ok(spell, host_worker_key);
        pop_failed(spell, host_key);

        // check remove
        push_ok(spell, host_worker_key, value);
        push_ok(spell, worker_key, value);

        remove_failed(spell, private_key);
        remove_ok(spell, worker_key);
        remove_ok(spell, host_worker_key);
        remove_failed(spell, host_key);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_list_other(spell: marine_test_env::spell::ModuleInterface) {
        let private_key = "key";
        let host_key = "h_key";
        let host_worker_key = "hw_key";
        let worker_key = "w_key";
        let value = "value";

        let spell = &mut spell;
        type SPELL = marine_test_env::spell::ModuleInterface;
        let cp = || other_call_params();
        let push_failed = |spell: &mut SPELL, key: &str, value: &str| {
            let list_before = spell.list_get_strings(key.into());
            let pushed = spell.list_push_string_cp(key.into(), value.into(), cp());
            assert!(!pushed.success, "pushing ({}, {})", key, value);
            let list_after = spell.list_get_strings(key.into());
            assert_eq!(list_before.value.len(), list_after.value.len());
        };
        let pop_failed = |spell: &mut SPELL, key: &str| {
            let popped = spell.list_pop_string_cp(key.into(), cp());
            assert!(!popped.success, "popping {}", key);
            let list = spell.list_get_strings(key.into());
            assert!(!list.value.is_empty());
        };
        let remove_failed = |spell: &mut SPELL, key: &str| {
            let removed = spell.list_remove_string_cp(key.into(), value.into(), cp());
            assert!(!removed.success, "removing ({}, {})", key, value);
            let list = spell.list_get_strings(key.into());
            assert!(!list.value.is_empty());
        };


        // check push
        push_failed(spell, private_key, value);
        push_failed(spell, worker_key, value);
        push_failed(spell, host_worker_key, value);
        push_failed(spell, host_key, value);

        // chek pop
        spell.list_push_string_cp(private_key.into(), value.into(), spell_call_params());
        spell.list_push_string_cp(host_key.into(), value.into(), spell_call_params());
        spell.list_push_string_cp(host_worker_key.into(), value.into(), spell_call_params());
        spell.list_push_string_cp(worker_key.into(), value.into(), spell_call_params());

        pop_failed(spell, private_key);
        pop_failed(spell, worker_key);
        pop_failed(spell, host_worker_key);
        pop_failed(spell, host_key);

        // check remove
        remove_failed(spell, private_key);
        remove_failed(spell, worker_key);
        remove_failed(spell, host_worker_key);
        remove_failed(spell, host_key);
    }

    fn spell_call_params() -> CallParameters {
        CallParameters {
            init_peer_id: "worker-id".to_string(),
            service_creator_peer_id: "worker-id".to_string(),
            particle_id: "spell_spell-id_0".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }

    fn host_call_params() -> CallParameters {
        CallParameters {
            init_peer_id: "host-id".to_string(),
            service_creator_peer_id: "worker-id".to_string(),
            particle_id: "some-particle".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }

    fn worker_call_params() -> CallParameters {
        CallParameters {
            init_peer_id: "worker-id".to_string(),
            service_creator_peer_id: "worker-id".to_string(),
            particle_id: "some-particle".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }

    fn other_call_params() -> CallParameters {
        CallParameters {
            init_peer_id: "other-worker-id".to_string(),
            service_creator_peer_id: "worker-id".to_string(),
            particle_id: "some-particle".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }
}
