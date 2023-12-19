use std::fs::OpenOptions;
use std::io::Write;

use marine_rs_sdk::marine;

use fluence_spell_dtos::value::{CIDv1Value, ScriptValue, UnitValue};

use crate::auth::is_by_creator;

const SCRIPT_FILE: &str = "/tmp/script.air";

#[marine]
pub fn set_script(script: String) -> UnitValue {
    if !is_by_creator() {
        return UnitValue::error("Only owner of the service can set the script");
    }

    // open file for writing, overwrite if exists, create if not exists
    let write = OpenOptions::new()
        // create file if it doesn't exist
        .create(true)
        // remove all contents of the file if it exists
        .truncate(true)
        // grant writing permissions
        .write(true)
        .open(SCRIPT_FILE)
        .map(|mut f| f.write_all(script.as_bytes()));

    match write {
        Ok(_) => UnitValue::ok(),
        Err(e) => UnitValue::error(format!("Error writing script to {}: {}", SCRIPT_FILE, e)),
    }
}

#[marine]
pub fn get_script() -> ScriptValue {
    match std::fs::read_to_string(SCRIPT_FILE) {
        Ok(value) => ScriptValue {
            value,
            success: true,
            error: <_>::default(),
        },
        Err(e) => ScriptValue {
            value: <_>::default(),
            success: false,
            error: e.to_string(),
        },
    }
}

const RAW: u64 = 0x55;

#[marine]
pub fn script_cid() -> CIDv1Value {
    use cid::multihash::{Code, MultihashDigest};
    use cid::Cid;

    let script = get_script();
    if script.success {
        let digest = Code::Sha2_256.digest(script.value.as_bytes());
        let cid = Cid::new_v1(RAW, digest);

        CIDv1Value {
            value: cid.to_string(),
            success: true,
            error: <_>::default(),
        }
    } else {
        CIDv1Value {
            value: <_>::default(),
            success: false,
            error: format!("error loading script: {}", script.error),
        }
    }
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;
    use marine_rs_sdk_test::CallParameters;

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

  #[marine_test(config_path = "../tests_artifacts/Config.toml")]
    fn test_set_script_source_to_file(spell: marine_test_env::spell::ModuleInterface) {
        assert!(
            spell
                .set_script("(null)".to_string())
                .success,
            "set_script_source_to_file returned false"
        );
        assert_eq!(spell.get_script().value, "(null)");
    }

  #[marine_test(config_path = "../tests_artifacts/Config.toml")]
    fn test_set_script_source_to_file_twice(spell: marine_test_env::spell::ModuleInterface) {
        assert!(
            spell
                .set_script("(null)".to_string())
                .success,
            "set_script_source_to_file returned false"
        );
        let second_set = spell.set_script("(seq (null) (null))".to_string());
        assert!(
            second_set.success,
            "set_script_source_to_file returned false (fail), expected true (success)"
        );
        assert_eq!(
            spell.get_script().value,
            "(seq (null) (null))"
        );
    }

  #[marine_test(config_path = "../tests_artifacts/Config.toml")]
    fn test_set_script_by_spell(spell: marine_test_env::spell::ModuleInterface) {
        let service_id = uuid::Uuid::new_v4();
        let particle_id = format!("spell_{}_123", service_id);

        let cp = CallParameters {
            init_peer_id: "folex".to_string(),
            service_creator_peer_id: "folex".to_string(),
            service_id: service_id.to_string(),
            host_id: "".to_string(),
            particle_id: particle_id,
            tetraplets: vec![],
        };

        let set = spell.set_script_cp("(null)".to_string(), cp);

        assert!(set.success, "set script failed: {}", set.error);
    }

  #[marine_test(config_path = "../tests_artifacts/Config.toml")]
    fn test_set_script_by_third_party(spell: marine_test_env::spell::ModuleInterface) {
        let cp = CallParameters {
            init_peer_id: "definitely not folex".to_string(),
            service_creator_peer_id: "folex".to_string(),
            service_id: "spell_service_id".to_string(),
            host_id: "".to_string(),
            particle_id: "some_particle_id_from_somewhere".to_string(),
            tetraplets: vec![],
        };

        let set = spell.set_script_cp("(null)".to_string(), cp);

        assert!(!set.success, "set script succeeded while shouldn't");
        assert_eq!(set.error, "Only owner of the service can set the script");
    }

  #[marine_test(config_path = "../tests_artifacts/Config.toml")]
    fn test_set_script_source_to_file_auth(spell: marine_test_env::spell::ModuleInterface) {
        let cp = CallParameters {
            init_peer_id: "folex".to_string(),
            service_creator_peer_id: "not folex".to_string(),
            service_id: "".to_string(),
            host_id: "".to_string(),
            particle_id: "".to_string(),
            tetraplets: vec![],
        };

        let set = spell.set_script_cp("(null)".to_string(), cp);

        assert!(!set.success, "set script succeeded while shouldn't");
        assert_eq!(set.error, "Only owner of the service can set the script");
    }

  #[marine_test(config_path = "../tests_artifacts/Config.toml")]
    fn test_cid(spell: marine_test_env::spell::ModuleInterface) {
        assert!(
            spell
                .set_script("(null)".to_string())
                .success,
            "set_script_source_to_file returned false"
        );
        assert_eq!(
            spell.script_cid().value,
            "bafkreibiotlyad7mvyqiit3ie2ljecziknctzuzmi7qtmktxmib5aiu3cq"
        );
    }
}
