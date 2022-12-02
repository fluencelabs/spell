use marine_rs_sdk::marine;
use marine_sqlite_connector::{State, Statement};

use fluence_spell_dtos::error::SpellError::*;
use fluence_spell_dtos::value::{StringValue, U32Value, UnitValue};

use crate::schema::db;

#[marine]
pub fn set_string(key: &str, value: String) -> UnitValue {
    let result: eyre::Result<()> = try {
        let mut statement = db().prepare("INSERT OR REPLACE INTO kv (key, string) VALUES (?, ?)")?;
        statement.bind(1, key)?;
        statement.bind(2, value.as_str())?;
        statement.next()?;
    };

    result.into()
}

pub fn read_string(statement: &mut Statement, key: &str, idx: usize) -> eyre::Result<String> {
    if let State::Row = statement.next()? {
        Ok(statement.read::<String>(idx)?)
    } else {
        Err(KeyNotExists(key.to_string()))?
    }
}

#[marine]
pub fn get_string(key: &str) -> StringValue {
    let result: eyre::Result<String> = try {
        let mut statement = db().prepare("SELECT string FROM kv WHERE key = ?")?;
        statement.bind(1, key)?;
        read_string(&mut statement, key, 0)?
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
    let result: eyre::Result<u32> = try {
        let mut statement = db().prepare("SELECT u32 FROM kv WHERE key = ?")?;
        statement.bind(1, key)?;
        if let State::Row = statement.next()? {
            statement.read::<f64>(0)? as u32
        } else {
            Err(KeyNotExists(key.to_string()))?
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

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[ctor::ctor]
    /// usage of 'ctor' makes this function run only once
    fn before_all_tests() {
        std::fs::remove_file("/tmp/spell_.sqlite").ok();
    }

    /// after_each macro copy-pastes this function into every test
    fn after_each() {
        std::fs::remove_file("/tmp/spell_.sqlite").ok();
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
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
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
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
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_remove_key(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num";
        let num = 123;

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
            assert!(!get.success);
            assert!(get
                .error
                .starts_with(format!("Key '{}' does not exist", key).as_str()));
        }
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
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
}
