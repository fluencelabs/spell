use marine_rs_sdk::{get_call_parameters, marine};
use marine_sqlite_connector::State;

use crate::auth::is_by_creator;
use crate::error::SpellError::NoRelay;
use crate::schema::db;
use crate::value::{LocationValue, UnitValue};

fn get_relay() -> eyre::Result<String> {
    let mut statement = db().prepare("SELECT relay FROM relay LIMIT 1")?;
    if let State::Row = statement.next()? {
        Ok(statement.read::<String>(0)?)
    } else {
        Err(NoRelay)?
    }
}

#[marine]
pub fn set_relay_peer_id(relay_peer_id: String) -> UnitValue {
    if !is_by_creator() {
        return UnitValue::error("Only owner of the service can set relay peer id");
    }

    if get_relay().is_ok() {
        return UnitValue::error("Relay was already set and cannot be changed");
    }

    let result: eyre::Result<()> = try {
        let mut statement = db().prepare(r#"INSERT INTO relay VALUES (?)"#)?;
        statement.bind(1, relay_peer_id.as_str())?;
        loop {
            match statement.next()? {
                State::Done => break,
                State::Row => continue
            }
        }
    };

    result.into()
}

#[marine]
pub fn get_location() -> LocationValue {
    match get_relay() {
        Ok(relay) => LocationValue::success(relay, get_call_parameters()),
        Err(e) => return LocationValue::error(e)
    }
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
    fn set_location(spell: marine_test_env::spell::ModuleInterface) {
        let set = spell.set_relay_peer_id("relay".into());
        assert!(set.success, "first set failed: {}", set.error);

        let set = spell.set_relay_peer_id("relay".into());
        assert_eq!(set.success, false, "second set succeeded: {}", set.error);

        let location = spell.get_location();
        assert!(location.success, "get_location failed: {}", location.error);
        assert_eq!(location.relay, "relay");
    }
}