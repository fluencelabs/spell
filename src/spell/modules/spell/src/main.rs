pub mod script;

extern crate core;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

use marine_sqlite_connector as sqlite;
use marine_sqlite_connector::{Connection, State, Statement, Value};

module_manifest!();

pub fn main() {
    // db()
    //     .execute(
    //         r#"
    //         CREATE TABLE IF NOT EXISTS script (script TEXT);
    //
    //         CREATE TRIGGER script_no_insert
    //             BEFORE INSERT ON script
    //             WHEN (SELECT COUNT(*) FROM script) >= 1   -- limit here
    //             BEGIN
    //                 SELECT RAISE(FAIL, 'there could be only a single script');
    //             END;
    //         "#,
    //     )
    //     .expect("init sqlite db");
}

fn db() -> Connection {
    sqlite::open("/tmp/spell.sqlite").expect("open sqlite db")
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../tests_artifacts/Config.toml", modules_dir = "../tests_artifacts")]
    fn test_set_script_source_to_file(spell: marine_test_env::spell::ModuleInterface) {
        assert!(spell.set_script_source_to_file("(null)".to_string()), "set_script_source_to_file returned false");
        assert_eq!(spell.get_script_source_from_file().source_code, "(null)");
    }
}