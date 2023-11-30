use fluence_spell_dtos::value::{StringListValue, StringValue, UnitValue};
use marine_rs_sdk::marine;
use marine_sqlite_connector::State;

use crate::misc::fetch_rows;
use crate::schema::db;

#[marine]
pub fn list_push_string(key: &str, value: String) -> UnitValue {
    let conn = db();
    let result: eyre::Result<()> = try {
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
    let db = db();
    let result: eyre::Result<Option<String>> = try {
        let mut get = db.prepare(
            r#"
            SELECT string, max(list_order) FROM kv
                WHERE key = ?
        "#,
        )?;
        get.bind(1, key)?;

        let mut result = None;

        if let State::Row = get.next()? {
            result = Some(get.read::<String>(0)?);
            let list_order = get.read::<i64>(1)?;

            let mut delete = db.prepare(r#"DELETE FROM kv WHERE key = ? AND list_order = ?"#)?;
            delete.bind(1, key)?;
            delete.bind(2, list_order)?;
            delete.next()?;
        }

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
                kv WHERE key = ?
            ORDER BY
                list_order ASC
        "#,
        )?;
        statement.bind(1, key)?;
        let list: Vec<String> = fetch_rows(statement, |statement| {
            Ok(Some(statement.read::<String>(0)?))
        });

        list
    };

    result.into()
}

#[marine]
/// Remove a value from a list of strings. If the value is in several places, remove all of them.
/// Returns an error on exceptions. Doesn't report if the value was actually removed.
pub fn list_remove_value(key: &str, value: &str) -> UnitValue {
    let conn = db();
    let result: eyre::Result<()> = try {
        // list_order == -1 when the key isn't a part of a list
        let mut statement = conn.prepare(
            r#"
            DELETE FROM kv
                WHERE key = ?
                  AND string = ?
                  AND list_order != -1
        "#,
        )?;
        statement.bind(1, key)?;
        statement.bind(2, value)?;
        statement.next()?;
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

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
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

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_push_pop_get(spell: marine_test_env::spell::ModuleInterface) {
        type SPELL = marine_test_env::spell::ModuleInterface;
        let key = "a";

        let check_len = |spell: &mut SPELL, len| {
            let get = spell.list_get_strings(key.into());
            assert!(get.success, "list_get_strings failed {}", get.error);
            assert_eq!(
                get.strings.len(),
                len,
                "expected {}, got {}",
                len,
                get.strings.len()
            );
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

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_list_remove(spell: marine_test_env::spell::ModuleInterface) {
        let key = "a";
        let value = "value_to_remove";

        let _ = spell.list_push_string(key.into(), "b".into());
        let _ = spell.list_push_string(key.into(), value.into());
        let _ = spell.list_push_string(key.into(), "в".into());
        let _ = spell.list_push_string(key.into(), "p".into());
        let _ = spell.list_push_string(key.into(), value.into());
        let _ = spell.list_push_string(key.into(), "a".into());

        let remove = spell.list_remove_value(key.into(), value.into());
        assert!(remove.success, "remove failed {}", remove.error);

        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.strings, vec!["b", "в", "p", "a"]);

        let remove = spell.list_remove_value(key.into(), "not-in-list".into());
        assert!(remove.success, "remove of non-existent values from the list must return ok");

        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.strings, vec!["b", "в", "p", "a"], "must be the same after removing of absent value");

        let _ = spell.set_string("str".into(), "val".into());
        let remove = spell.list_remove_value("str".into(), "val".into());
        assert!(remove.success, "remove of non-existent key-values from a list must return ok");

        let get = spell.get_string("str".into());
        assert!(!get.absent, "non-list value must not be removed");
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_list_remove_and_add(spell: marine_test_env::spell::ModuleInterface) {
        let key = "a";
        let value = "value_to_remove";
        let value_first = "value_first";

        let _ = spell.list_push_string(key.into(), value_first.into());
        let _ = spell.list_push_string(key.into(), value.into());
        let _ = spell.list_push_string(key.into(), "в".into());
        let _ = spell.list_push_string(key.into(), "p".into());
        let _ = spell.list_push_string(key.into(), value.into());
        let _ = spell.list_push_string(key.into(), "a".into());

        let _ = spell.list_remove_value(key.into(), value.into());

        let _ = spell.list_push_string(key.into(), "n1".into());
        let _ = spell.list_push_string(key.into(), "n2".into());

        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.strings, vec![value_first, "в", "p", "a", "n1", "n2"]);

        let _ = spell.list_remove_value(key.into(), value_first.into());
        let get = spell.list_get_strings(key.into());
        assert!(get.success, "list_get_strings failed {}", get.error);
        assert_eq!(get.strings, vec!["в", "p", "a", "n1", "n2"]);

    }

}
