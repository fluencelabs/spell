use eyre::WrapErr;
use marine_rs_sdk::marine;
use marine_sqlite_connector::State;

use crate::kv::read_string;
use crate::value::{StringListValue, StringValue, UnitValue};
use crate::schema::db;

#[marine]
pub fn list_push_string(key: &str, value: String) -> UnitValue {
    let result: eyre::Result<()> = try {
        let mut statement = db().prepare(
            r#"
                INSERT INTO kv (key, string, list_index)
                    VALUES (
                        ?,
                        ?,
                        (SELECT COUNT(*) FROM kv WHERE key = ?)
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
    let db = db();
    let result: eyre::Result<String> = try {
        let mut get = db.prepare(r#"
            SELECT * FROM kv
                WHERE key = ?
                AND list_index = ((SELECT COUNT(*) FROM kv WHERE key = ?) - 1)
        "#)?;
        get.bind(1, key)?;
        get.bind(2, key)?;
        let string = read_string(&mut get, key, 1)?;
        let list_index = get.read::<f64>(3)?;

        let mut delete = db.prepare(
            r#"
            DELETE FROM kv
                WHERE key = ? AND list_index = ?
        "#,
        )?;
        delete.bind(1, key)?;
        delete.bind(2, list_index)?;
        delete.next()?;

        string
    };

    result.into()
}

#[marine]
/// Get a whole list of strings
pub fn list_get_strings(key: &str) -> StringListValue {
    let result: eyre::Result<Vec<String>> = try {
        let mut statement = db().prepare(
            r#"
            SELECT
                string
            FROM
                kv WHERE key = ?
        "#,
        )?;
        statement.bind(1, key)?;
        std::iter::from_fn(move || {
            let r: eyre::Result<Option<String>> = try {
                if let State::Row = statement.next()? {
                    Some(statement.read::<String>(0)?)
                } else {
                    None
                }
            };
            r.context("error fetching error row from sqlite")
                .transpose()
        })
        .filter_map(|r| r.ok())
        .collect()
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
        std::fs::remove_file("/tmp/spell.sqlite").ok();
    }

    /// after_each macro copy-pastes this function into every test
    fn after_each() {
        std::fs::remove_file("/tmp/spell.sqlite").ok();
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_push_get(spell: marine_test_env::spell::ModuleInterface) {
        let key = "a";

        let push = spell.list_push_string(key.into(), "b".into());
        assert!(push.success, "1 push failed {}", push.error);
        let push = spell.list_push_string(key.into(), "f".into());
        assert!(push.success, "2 push failed {}", push.error);
        let push = spell.list_push_string(key.into(), "в".into());
        assert!(push.success, "3 push failed {}", push.error);

        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.strings, vec!["b", "f", "в"]);
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_push_pop_get(spell: marine_test_env::spell::ModuleInterface) {
        type SPELL = marine_test_env::spell::ModuleInterface;
        let key = "a";

        let check_len = |spell: &mut SPELL, len| {
            let get = spell.list_get_strings(key.into());
            assert!(get.success, "list_get_strings failed {}", get.error);
            assert_eq!(get.strings.len(), len, "expected {}, got {}", len, get.strings.len());
        };

        let pop = |spell: &mut SPELL, expectation| {
            let pop = spell.list_pop_string(key.into());
            assert!(pop.success, "pop failed {}", pop.error);
            assert_eq!(pop.str, expectation);
        };

        let push = |spell: &mut SPELL, value: &str| {
            let push = spell.list_push_string(key.into(), value.into());
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
        assert_eq!(get.strings, &["хаха", "хаха", "haha"]);
    }
}
