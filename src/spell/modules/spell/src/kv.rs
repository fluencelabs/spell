use marine_rs_sdk::marine;
use marine_sqlite_connector::State;

use crate::error::SpellError::*;
use crate::result::UnitResult;
use crate::schema::db;

#[marine]
pub struct StringValue {
    pub str: String,
    pub success: bool,
    pub error: String,
}

#[marine]
pub struct U32Value {
    pub num: u32,
    pub success: bool,
    pub error: String,
}

#[marine]
pub fn set_string(key: &str, value: String) -> UnitResult {
    let result: eyre::Result<()> = try {
        let mut statement = db().prepare("INSERT INTO kv (key, string) VALUES (?, ?)")?;
        statement.bind(1, key)?;
        statement.bind(2, value.as_str())?;
        statement.next()?;
    };

    match result {
        Ok(_) => UnitResult::ok(),
        Err(e) => UnitResult::error(e),
    }
}

#[marine]
pub fn get_string(key: &str) -> StringValue {
    let result: eyre::Result<String> = try {
        let mut statement = db().prepare("SELECT string FROM kv WHERE key = ?")?;
        statement.bind(1, key)?;
        if let State::Row = statement.next()? {
            statement.read::<String>(0)?
        } else {
            Err(KeyNotExists(key.to_string()))?
        }
    };

    match result {
        Ok(str) => StringValue {
            str,
            success: true,
            error: <_>::default(),
        },
        Err(e) => StringValue {
            str: <_>::default(),
            success: false,
            error: e.to_string(),
        },
    }
}

#[marine]
pub fn set_u32(key: &str, value: u32) -> UnitResult {
    let result: eyre::Result<()> = try {
        let mut statement = db().prepare("INSERT INTO kv (key, u32) VALUES (?, ?)")?;
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

    match result {
        Ok(num) => U32Value {
            num,
            success: true,
            error: <_>::default(),
        },
        Err(e) => U32Value {
            num: <_>::default(),
            success: false,
            error: e.to_string(),
        },
    }
}

#[marine]
/// Deletes a key (and associated value) from K/V.
/// Always succeeds.
pub fn remove_key(key: &str) -> UnitResult {
    let result: eyre::Result<()> = try {
        let mut statement = db().prepare("DELETE FROM kv WHERE key = ?")?;
        statement.bind(1, key)?;
        statement.next()?;
    };

    result.into()
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

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
            assert_eq!(get.error, format!("Key '{}' does not exist", key));
        }
    }
}
