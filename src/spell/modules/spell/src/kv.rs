use marine_rs_sdk::marine;
use marine_sqlite_connector as sqlite;
use marine_sqlite_connector::{Connection, State}; //, Statement, Value};
use crate::error::SpellError::*;

pub fn db() -> Connection {
    sqlite::open("/tmp/spell.sqlite").expect("open sqlite db")
}

pub fn create_db() {
    db()
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS kv (key TEXT, string TEXT, u32 INTEGER);
            "#,
        )
        .expect("init sqlite db");
}

#[marine]
pub struct StringValue {
    pub str: String,
    pub success: bool,
    pub error: String
}

#[marine]
pub struct U32Value {
    pub num: u32,
    pub success: bool,
    pub error: String
}

#[marine]
pub struct SetResult {
    pub success: bool,
    pub error: String
}

#[marine]
pub fn set_string(key: String, value: String) -> SetResult {
    let result: anyhow::Result<()> = try {
        let mut statement = db().prepare("INSERT INTO kv (key, string) VALUES (?, ?)")?;
        statement.bind(1, key.as_str())?;
        statement.bind(2, value.as_str())?;
        statement.next()?;
    };

    match result {
        Ok(_) => SetResult { success: true, error: <_>::default() },
        Err(e) => SetResult { success: false, error: e.to_string() },
    }
}

#[marine]
pub fn get_string(key: String) -> StringValue {
    let result: anyhow::Result<String> = try {
        let mut statement = db().prepare("SELECT string FROM kv WHERE key = ?")?;
        statement.bind(1, key.as_str())?;
        if let State::Row = statement.next()? {
            statement.read::<String>(0)?
        } else {
            Err(KeyNotExists(key))?
        }
    };

    match result {
        Ok(str) => StringValue { str, success: true, error: <_>::default() },
        Err(e) => StringValue { str: <_>::default(), success: false, error: e.to_string() },
    }
}

#[marine]
pub fn set_u32(key: String, value: u32) -> SetResult {
    let result: anyhow::Result<()> = try {
        let mut statement = db().prepare("INSERT INTO kv (key, u32) VALUES (?, ?)")?;
        statement.bind(1, key.as_str())?;
        statement.bind(2, value as f64)?;
        statement.next()?;
    };

    match result {
        Ok(_) => SetResult { success: true, error: <_>::default() },
        Err(e) => SetResult { success: false, error: e.to_string() },
    }
}

#[marine]
pub fn get_u32(key: String) -> U32Value {
    let result: anyhow::Result<u32> = try {
        let mut statement = db().prepare("SELECT u32 FROM kv WHERE key = ?")?;
        statement.bind(1, key.as_str())?;
        if let State::Row = statement.next()? {
            statement.read::<f64>(0)? as u32
        } else {
            Err(KeyNotExists(key))?
        }
    };

    match result {
        Ok(num) => U32Value { num, success: true, error: <_>::default() },
        Err(e) => U32Value { num: <_>::default(), success: false, error: e.to_string() },
    }
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../tests_artifacts/Config.toml", modules_dir = "../tests_artifacts")]
    fn test_string(spell: marine_test_env::spell::ModuleInterface) {
        let key = "str".to_string();
        let str = "b".to_string();
        let set = spell.set_string(key.clone(), str);
        assert!(set.success, "set_string failed: {}", set.error);
        let get = spell.get_string(key);
        assert_eq!(get.str, "b", "get_string failed: {}", get.error);
    }

    #[marine_test(config_path = "../tests_artifacts/Config.toml", modules_dir = "../tests_artifacts")]
    fn test_u32(spell: marine_test_env::spell::ModuleInterface) {
        let key = "num".to_string();
        let num = 123;
        let set = spell.set_u32(key.clone(), num);
        assert!(set.success, "set_u32 failed: {}", set.error);
        let get = spell.get_u32(key);
        assert_eq!(get.num, num, "get_u32 failed: {}", get.error);
    }
}