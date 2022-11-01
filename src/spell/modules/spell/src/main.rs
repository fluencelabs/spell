pub mod script;

extern crate core;

// use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

use marine_sqlite_connector as sqlite;
use marine_sqlite_connector::Connection; //, State, Statement, Value};

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

#[allow(unused)]
fn db() -> Connection {
    sqlite::open("/tmp/spell.sqlite").expect("open sqlite db")
}
