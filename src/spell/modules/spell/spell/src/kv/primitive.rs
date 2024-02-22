use marine_rs_sdk::marine;
use marine_sqlite_connector::{State, Statement};

use crate::auth::guard_kv_write;
use fluence_spell_dtos::value::{BoolValue, StringValue, U32Value, UnitValue};

use crate::schema::db;

//
// Note that it's possible to call this function on an empty string value, but it will be stored as a NULL value
// in the database since SQLite connector we use don't save an empty string as an empty string which IS possible
// if you manually try to do so. I didn't found WHY it's happening, but it's not a really big deal, just annoying.
//
pub fn store_string(key: &str, value: String) -> eyre::Result<()> {
    let conn = db();
    let mut statement = conn.prepare("INSERT OR REPLACE INTO kv (key, string) VALUES (?, ?)")?;
    statement.bind(1, key)?;
    statement.bind(2, value.as_str())?;
    statement.next()?;

    Ok(())
}

#[marine]
pub fn set_string(key: &str, value: String) -> UnitValue {
    let result: eyre::Result<()> = try {
        guard_kv_write(key)?;
        store_string(key, value)?
    };
    result.into()
}

pub fn read_string(statement: &mut Statement, idx: usize) -> eyre::Result<Option<String>> {
    if let State::Row = statement.next()? {
        let read_value = statement.read::<String>(idx)?;
        // Need to clone because otherwise `Some(read_value)` morphs into `None` O.o
        Ok(Some(read_value.to_string()))
    } else {
        Ok(None)
    }
}

#[marine]
pub fn get_string(key: &str) -> StringValue {
    let result: eyre::Result<Option<String>> = try {
        let conn = db();
        // As long as an empty string is saved a NULL value, we can determine that the value is a string
        // by checking that the other possible type, u32, is null.
        // list_order == -1 when the value isn't part of the list
        let mut statement = conn.prepare(
            r#"
            SELECT string
              FROM kv
             WHERE key = ?
               AND u32 IS NULL
               AND list_order == -1
            "#,
        )?;
        statement.bind(1, key)?;
        read_string(&mut statement, 0)?
    };
    result.into()
}

#[marine]
pub fn set_u32(key: &str, value: u32) -> UnitValue {
    let result: eyre::Result<()> = try {
        guard_kv_write(key)?;
        let conn = db();
        let mut statement = conn.prepare("INSERT OR REPLACE INTO kv (key, u32) VALUES (?, ?)")?;
        statement.bind(1, key)?;
        statement.bind(2, value as i64)?;
        statement.next()?;
    };

    result.into()
}

fn read_u32(statement: &mut Statement) -> eyre::Result<Option<u32>> {
    if let State::Row = statement.next()? {
        let read_value = statement.read::<i64>(0)?;
        Ok(Some(read_value as u32))
    } else {
        Ok(None)
    }
}

#[marine]
pub fn get_u32(key: &str) -> U32Value {
    let result: eyre::Result<Option<u32>> = try {
        let conn = db();
        // list_order == -1 when the value isn't part of the list
        // when u32 is NULL it means that the value is a string or a list
        let mut statement = conn.prepare(
            r#"
            SELECT u32
              FROM kv
             WHERE key = ?
               AND u32 IS NOT NULL
               AND list_order == -1
            "#,
        )?;
        statement.bind(1, key)?;
        read_u32(&mut statement)?
    };

    result.into()
}

#[marine]
/// Deletes a key (and associated value/lists) from K/V.
/// Always succeeds.
pub fn remove_key(key: &str) -> UnitValue {
    let result: eyre::Result<()> = try {
        guard_kv_write(key)?;
        let conn = db();
        let mut statement = conn.prepare("DELETE FROM kv WHERE key = ?")?;
        statement.bind(1, key)?;
        statement.next()?;
    };

    result.into()
}

#[marine]
pub fn exists(key: &str) -> BoolValue {
    let result: eyre::Result<bool> = try {
        let conn = db();
        let mut statement = conn.prepare("SELECT 1 FROM kv WHERE key = ? LIMIT 1")?;
        statement.bind(1, key)?;

        match statement.next()? {
            State::Row => true,
            State::Done => false,
        }
    };

    result.into()
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk::ParticleParameters;
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
    fn test_string(spell: marine_test_env::spell::ModuleInterface) {
        let cp = spell_call_params();

        let key = "str".to_string();
        let str = "b".to_string();
        let set = spell.set_string_cp(key.clone(), str, cp);
        assert!(set.success, "set_string failed: {}", set.error);
        let get = spell.get_string(key);
        assert_eq!(get.value, "b", "get_string failed: {}", get.error);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_u32(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num".to_string();
        let num = 123;
        let set = spell.set_u32_cp(key.clone(), num, spell_call_params());
        assert!(set.success, "set_u32 failed: {}", set.error);
        let get = spell.get_u32(key);
        assert_eq!(get.value, num, "get_u32 failed: {}", get.error);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_remove_key(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num";
        let num = 123;
        let cp = || spell_call_params();

        let get = spell.get_u32(key.into());
        assert!(get.success, "unable to retrieve key {}: {}", key, get.error);
        assert!(get.absent, "key {} exists", key);
        assert!(
            get.error.is_empty(),
            "there should be no error when value is absent"
        );

        for _ in 1..10 {
            let set = spell.set_u32_cp(key.into(), num, cp());
            assert!(set.success, "set_u32 failed: {}", set.error);

            let get = spell.get_u32(key.into());
            assert_eq!(get.value, num, "get_u32 failed: {}", get.error);

            let remove = spell.remove_key_cp(key.into(), cp());
            assert!(remove.success, "first remove failed: {}", remove.error);

            let remove = spell.remove_key_cp(key.into(), cp());
            assert!(remove.success, "second remove failed: {}", remove.error);

            let get = spell.get_u32(key.into());
            assert!(get.success, "unable to retrieve key {}: {}", key, get.error);
            assert!(get.absent, "key {} still exists", key);
            assert!(
                get.error.is_empty(),
                "there should be no error when value is absent"
            );
        }
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_u32_mutate(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num".to_string();
        let num = 123;
        let cp = || spell_call_params();

        let set = spell.set_u32_cp(key.clone(), num, cp());
        assert!(set.success, "set_u32 failed: {}", set.error);
        let get = spell.get_u32(key.clone());
        assert_eq!(get.value, num, "get_u32 failed: {}", get.error);

        let set = spell.set_u32_cp(key.clone(), num * 2, cp());
        assert!(set.success, "set_u32 failed: {}", set.error);

        let get = spell.get_u32(key);
        assert_eq!(get.value, num * 2, "get_u32 failed: {}", get.error);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_exists(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num".to_string();
        let cp = || spell_call_params();

        // check if exists before insertion
        let exists = spell.exists(key.clone());
        assert!(exists.success, "first exists failed: {}", exists.error);
        assert!(!exists.value, "value exists before set");

        // insert
        let num = 123;
        let set = spell.set_u32_cp(key.clone(), num, cp());
        assert!(set.success, "set_u32 failed: {}", set.error);

        // check if exists after insertion
        let exists = spell.exists(key.clone());
        assert!(exists.success, "second exists failed: {}", exists.error);
        assert!(exists.value, "value doesn't exists after set_u32");

        // remove
        let remove = spell.remove_key_cp(key.clone(), cp());
        assert!(remove.success, "remove failed: {}", remove.error);

        // check if exists after remove
        let exists = spell.exists(key.clone());
        assert!(exists.success, "third exists failed: {}", exists.error);
        assert!(!exists.value, "value still exists after remove_key");
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_exists_empty_key(spell: marine_test_env::spell::ModuleInterface) {
        let exists = spell.exists(String::new());
        assert!(exists.success, "exists failed: {}", exists.error);
        assert!(!exists.value, "empty key exists");
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_set_same_key(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num".to_string();
        let num = 123;
        let str = "value".to_string();
        let cp = || spell_call_params();

        let set_num = spell.set_u32_cp(key.clone(), num, cp());
        assert!(set_num.success, "set_u32 failed: {}", set_num.error);

        let get_str = spell.get_string(key.clone());
        assert!(
            get_str.success,
            "get_string shouldn't fail: {}",
            get_str.error
        );
        assert!(get_str.absent, "the value of the wrong type must be absent");

        let set_str = spell.set_string_cp(key.clone(), str, cp());
        assert!(set_str.success, "set_u32 failed: {}", set_str.error);

        let get_num = spell.get_u32(key.clone());
        assert!(
            get_num.success,
            "get_string shouldn't fail: {}",
            get_num.error
        );
        assert!(get_num.absent, "the value of the wrong type must be absent");
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_set_empty_string(spell: marine_test_env::spell::ModuleInterface) {
        let key = "str".to_string();
        let str = String::new();

        let set = spell.set_string_cp(key.clone(), str, spell_call_params());
        assert!(set.success, "set_string failed: {}", set.error);

        let get = spell.get_string(key);
        assert!(get.success, "get_string failed: {}", get.error);
        assert!(
            !get.absent,
            "get_string must return non-absent empty string"
        );
        assert_eq!(get.value, "", "get_string failed: {}", get.error);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_set_host(spell: marine_test_env::spell::ModuleInterface) {
        let host_key = "h_str";
        let worker_key = "w_str";
        let host_worker_key = "hw_str";
        let private_key = "str";
        let value_str = "b".to_string();
        let value_num = 3;
        let cp = || host_call_params();

        type SPELL = marine_test_env::spell::ModuleInterface;
        let set_str_ok = |spell: &mut SPELL, key: &str, value: &str| {
            let set = spell.set_string_cp(key.into(), value.into(), cp());
            assert!(set.success, "set_string failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(exist.value);
        };
        let set_str_failed = |spell: &mut SPELL, key: &str, value: &str| {
            let set = spell.set_string_cp(key.into(), value.into(), cp());
            assert!(!set.success, "set_string failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(!exist.value);
        };
        let set_num_ok = |spell: &mut SPELL, key: &str, value: u32| {
            let set = spell.set_u32_cp(key.into(), value, cp());
            assert!(set.success, "set_u32 failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(exist.value);
        };
        let set_num_failed = |spell: &mut SPELL, key: &str, value: u32| {
            let set = spell.set_u32_cp(key.into(), value, cp());
            assert!(!set.success, "set_u32 failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(!exist.value);
        };
        let remove_ok = |spell: &mut SPELL, key: &str| {
            let remove = spell.remove_key_cp(key.into(), cp());
            assert!(remove.success, "remove failed: {}", remove.error);
            let exist = spell.exists(key.into());
            assert!(!exist.value);
        };
        let remove_failed = |spell: &mut SPELL, key: &str| {
            let remove = spell.remove_key_cp(key.into(), cp());
            assert!(!remove.success, "remove failed: {}", remove.error);
            let exist = spell.exists(key.into());
            assert!(exist.value);
        };

        set_str_ok(&mut spell, host_key, &value_str);
        set_str_ok(&mut spell, host_worker_key, &value_str);
        set_str_failed(&mut spell, worker_key, &value_str);
        set_str_failed(&mut spell, private_key, &value_str);

        remove_ok(&mut spell, host_key);
        remove_ok(&mut spell, host_worker_key);

        set_num_ok(&mut spell, host_key, value_num);
        set_num_ok(&mut spell, host_worker_key, value_num);
        set_num_failed(&mut spell, worker_key, value_num);
        set_num_failed(&mut spell, private_key, value_num);

        spell.set_u32_cp(worker_key.into(), value_num, spell_call_params());
        spell.set_u32_cp(private_key.into(), value_num, spell_call_params());

        remove_ok(&mut spell, host_key);
        remove_ok(&mut spell, host_worker_key);
        remove_failed(&mut spell, worker_key);
        remove_failed(&mut spell, private_key);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_set_worker(spell: marine_test_env::spell::ModuleInterface) {
        let host_key = "h_str";
        let worker_key = "w_str";
        let host_worker_key = "hw_str";
        let private_key = "str";
        let value_str = "b".to_string();
        let value_num = 3;
        let cp = || worker_call_params();

        type SPELL = marine_test_env::spell::ModuleInterface;
        let set_str_ok = |spell: &mut SPELL, key: &str, value: &str| {
            let set = spell.set_string_cp(key.into(), value.into(), cp());
            assert!(set.success, "set_string failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(exist.value);
        };
        let set_str_failed = |spell: &mut SPELL, key: &str, value: &str| {
            let set = spell.set_string_cp(key.into(), value.into(), cp());
            assert!(!set.success, "set_string failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(!exist.value);
        };
        let set_num_ok = |spell: &mut SPELL, key: &str, value: u32| {
            let set = spell.set_u32_cp(key.into(), value, cp());
            assert!(set.success, "set_u32 failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(exist.value);
        };
        let set_num_failed = |spell: &mut SPELL, key: &str, value: u32| {
            let set = spell.set_u32_cp(key.into(), value, cp());
            assert!(!set.success, "set_u32 failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(!exist.value);
        };
        let remove_ok = |spell: &mut SPELL, key: &str| {
            let remove = spell.remove_key_cp(key.into(), cp());
            assert!(remove.success, "remove failed: {}", remove.error);
            let exist = spell.exists(key.into());
            assert!(!exist.value);
        };
        let remove_failed = |spell: &mut SPELL, key: &str| {
            let remove = spell.remove_key_cp(key.into(), cp());
            assert!(!remove.success, "remove failed: {}", remove.error);
            let exist = spell.exists(key.into());
            assert!(exist.value);
        };

        set_str_failed(&mut spell, host_key, &value_str);
        set_str_ok(&mut spell, host_worker_key, &value_str);
        set_str_ok(&mut spell, worker_key, &value_str);
        set_str_failed(&mut spell, private_key, &value_str);

        remove_ok(&mut spell, worker_key);
        remove_ok(&mut spell, host_worker_key);

        set_num_failed(&mut spell, host_key, value_num);
        set_num_ok(&mut spell, host_worker_key, value_num);
        set_num_ok(&mut spell, worker_key, value_num);
        set_num_failed(&mut spell, private_key, value_num);

        spell.set_u32_cp(host_key.into(), value_num, spell_call_params());
        spell.set_u32_cp(private_key.into(), value_num, spell_call_params());

        remove_failed(&mut spell, host_key);
        remove_ok(&mut spell, host_worker_key);
        remove_ok(&mut spell, worker_key);
        remove_failed(&mut spell, private_key);
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_primitives_other(spell: marine_test_env::spell::ModuleInterface) {
        let host_key = "h_str";
        let worker_key = "w_str";
        let host_worker_key = "hw_str";
        let private_key = "str";
        let value_str = "b".to_string();
        let value_num = 3;
        let cp = || other_call_params();

        type SPELL = marine_test_env::spell::ModuleInterface;
        let set_str_failed = |spell: &mut SPELL, key: &str, value: &str| {
            let set = spell.set_string_cp(key.into(), value.into(), cp());
            assert!(!set.success, "set_string failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(!exist.value);
        };
        let set_num_failed = |spell: &mut SPELL, key: &str, value: u32| {
            let set = spell.set_u32_cp(key.into(), value, cp());
            assert!(!set.success, "set_u32 failed: {}", set.error);
            let exist = spell.exists(key.into());
            assert!(!exist.value);
        };
        let remove_failed = |spell: &mut SPELL, key: &str| {
            let remove = spell.remove_key_cp(key.into(), cp());
            assert!(!remove.success, "remove failed: {}", remove.error);
            let exist = spell.exists(key.into());
            assert!(exist.value);
        };

        set_str_failed(&mut spell, host_key, &value_str);
        set_str_failed(&mut spell, host_worker_key, &value_str);
        set_str_failed(&mut spell, worker_key, &value_str);
        set_str_failed(&mut spell, private_key, &value_str);

        set_num_failed(&mut spell, host_key, value_num);
        set_num_failed(&mut spell, host_worker_key, value_num);
        set_num_failed(&mut spell, worker_key, value_num);
        set_num_failed(&mut spell, private_key, value_num);

        spell.set_u32_cp(host_key.into(), value_num, spell_call_params());
        spell.set_u32_cp(private_key.into(), value_num, spell_call_params());
        spell.set_u32_cp(worker_key.into(), value_num, spell_call_params());
        spell.set_u32_cp(host_worker_key.into(), value_num, spell_call_params());

        remove_failed(&mut spell, host_key);
        remove_failed(&mut spell, host_worker_key);
        remove_failed(&mut spell, worker_key);
        remove_failed(&mut spell, private_key);
    }
    fn spell_call_params() -> CallParameters {
        CallParameters {
            particle: ParticleParameters {
                init_peer_id: "worker-id".to_string(),
                id: "spell_spell-id_0".to_string(),
                ..<_>::default()
            },
            service_creator_peer_id: "worker-id".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }

    fn host_call_params() -> CallParameters {
        CallParameters {
            particle: ParticleParameters {
                init_peer_id: "host-id".to_string(),
                id: "some-particle".to_string(),
                ..<_>::default()
            },
            service_creator_peer_id: "worker-id".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }

    fn worker_call_params() -> CallParameters {
        CallParameters {
            particle: ParticleParameters {
                init_peer_id: "worker-id".to_string(),
                id: "some-particle".to_string(),
                ..<_>::default()
            },
            service_creator_peer_id: "worker-id".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }

    fn other_call_params() -> CallParameters {
        CallParameters {
            particle: ParticleParameters {
                init_peer_id: "other-worker-id".to_string(),
                id: "some-particle".to_string(),
                ..<_>::default()
            },
            service_creator_peer_id: "worker-id".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }
}
