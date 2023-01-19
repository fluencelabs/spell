use marine_rs_sdk::marine;
use marine_sqlite_connector::{State, Statement};

use fluence_spell_dtos::value::{BoolValue, StringValue, U32Value, UnitValue};

use crate::schema::db;

pub fn store_string(key: &str, value: String) -> eyre::Result<()> {
    let mut statement = db().prepare("INSERT OR REPLACE INTO kv (key, string) VALUES (?, ?)")?;
    statement.bind(1, key)?;
    statement.bind(2, value.as_str())?;
    statement.next()?;

    Ok(())
}

#[marine]
pub fn set_string(key: &str, value: String) -> UnitValue {
    store_string(key, value).into()
}

pub fn read_string(statement: &mut Statement, idx: usize) -> eyre::Result<Option<String>> {
    if let State::Row = statement.next()? {
        Ok(Some(statement.read::<String>(idx)?))
    } else {
        Ok(None)
    }
}

#[marine]
pub fn get_string(key: &str) -> StringValue {
    let result: eyre::Result<Option<String>> = try {
        let mut statement = db().prepare("SELECT string FROM kv WHERE key = ?")?;
        statement.bind(1, key)?;
        read_string(&mut statement, 0)?
    };

    result.into()
}

#[marine]
pub fn set_u32(key: &str, value: u32) -> UnitValue {
    let result: eyre::Result<()> = try {
        let mut statement = db().prepare("INSERT OR REPLACE INTO kv (key, u32) VALUES (?, ?)")?;
        statement.bind(1, key)?;
        statement.bind(2, value as f64)?;
        statement.next()?;
    };

    result.into()
}

#[marine]
pub fn get_u32(key: &str) -> U32Value {
    let result: eyre::Result<Option<u32>> = try {
        let mut statement = db().prepare("SELECT u32 FROM kv WHERE key = ?")?;
        statement.bind(1, key)?;
        if let State::Row = statement.next()? {
            Some(statement.read::<f64>(0)? as u32)
        } else {
            None
        }
    };

    result.into()
}

#[marine]
/// Deletes a key (and associated value) from K/V.
/// Always succeeds.
pub fn remove_key(key: &str) -> UnitValue {
    let result: eyre::Result<()> = try {
        let mut statement = db().prepare("DELETE FROM kv WHERE key = ?")?;
        statement.bind(1, key)?;
        statement.next()?;
    };

    result.into()
}

#[marine]
pub fn exists(key: &str) -> BoolValue {
    let result: eyre::Result<bool> = try {
        let mut statement = db().prepare("SELECT 1 FROM kv WHERE key = ? LIMIT 1")?;
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

    #[marine_test(
        config_path = "../../tests_artifacts/Config.toml",
        modules_dir = "../../tests_artifacts"
    )]
    fn test_string(spell: marine_test_env::spell::ModuleInterface) {
        let key = "str".to_string();
        let str = "b".to_string();
        let set = spell.set_string(key.clone(), str);
        assert!(set.success, "set_string failed: {}", set.error);
        let get = spell.get_string(key);
        assert_eq!(get.str, "b", "get_string failed: {}", get.error);
    }

    #[marine_test(
        config_path = "../../tests_artifacts/Config.toml",
        modules_dir = "../../tests_artifacts"
    )]
    fn test_u32(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num".to_string();
        let num = 123;
        let set = spell.set_u32(key.clone(), num);
        assert!(set.success, "set_u32 failed: {}", set.error);
        let get = spell.get_u32(key);
        assert_eq!(get.num, num, "get_u32 failed: {}", get.error);
    }

    #[marine_test(
        config_path = "../../tests_artifacts/Config.toml",
        modules_dir = "../../tests_artifacts"
    )]
    fn test_remove_key(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num";
        let num = 123;

        let get = spell.get_u32(key.into());
        assert!(get.success, "unable to retrieve key {}: {}", key, get.error);
        assert!(get.absent, "key {} exists", key);
        assert!(
            get.error.is_empty(),
            "there should be no error when value is absent"
        );

        for _ in 1..10 {
            let set = spell.set_u32(key.into(), num);
            assert!(set.success, "set_u32 failed: {}", set.error);

            let get = spell.get_u32(key.into());
            assert_eq!(get.num, num, "get_u32 failed: {}", get.error);

            let remove = spell.remove_key(key.into());
            assert!(remove.success, "first remove failed: {}", remove.error);

            let remove = spell.remove_key(key.into());
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

    #[marine_test(
        config_path = "../../tests_artifacts/Config.toml",
        modules_dir = "../../tests_artifacts"
    )]
    fn test_u32_mutate(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num".to_string();
        let num = 123;
        let set = spell.set_u32(key.clone(), num);
        assert!(set.success, "set_u32 failed: {}", set.error);
        let get = spell.get_u32(key.clone());
        assert_eq!(get.num, num, "get_u32 failed: {}", get.error);

        let set = spell.set_u32(key.clone(), num * 2);
        assert!(set.success, "set_u32 failed: {}", set.error);

        let get = spell.get_u32(key);
        assert_eq!(get.num, num * 2, "get_u32 failed: {}", get.error);
    }

    #[marine_test(
        config_path = "../../tests_artifacts/Config.toml",
        modules_dir = "../../tests_artifacts"
    )]
    fn test_exists(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num".to_string();

        // check if exists before insertion
        let exists = spell.exists(key.clone());
        assert!(exists.success, "first exists failed: {}", exists.error);
        assert!(!exists.flag, "value exists before set");

        // insert
        let num = 123;
        let set = spell.set_u32(key.clone(), num);
        assert!(set.success, "set_u32 failed: {}", set.error);

        // check if exists after insertion
        let exists = spell.exists(key.clone());
        assert!(exists.success, "second exists failed: {}", exists.error);
        assert!(exists.flag, "value doesn't exists after set_u32");

        // remove
        let remove = spell.remove_key(key.clone());
        assert!(remove.success, "remove failed: {}", remove.error);

        // check if exists after remove
        let exists = spell.exists(key.clone());
        assert!(exists.success, "third exists failed: {}", exists.error);
        assert!(!exists.flag, "value still exists after remove_key");
    }

    #[marine_test(
        config_path = "../../tests_artifacts/Config.toml",
        modules_dir = "../../tests_artifacts"
    )]
    fn test_exists_empty_key(spell: marine_test_env::spell::ModuleInterface) {
        let exists = spell.exists(String::new());
        assert!(exists.success, "exists failed: {}", exists.error);
        assert!(!exists.flag, "empty key exists");
    }
}
