use std::fs::OpenOptions;
use std::io;
use std::io::Write;

use marine_rs_sdk::marine;

use fluence_spell_dtos::value::UnitValue;

use crate::auth::is_by_creator;

const SCRIPT_ENV: &str = "script";
const SCRIPT_FILE: &str = "/tmp/script.air";

#[allow(unused)]
fn check_env() {
    if let Err(e) = std::env::var(SCRIPT_ENV) {
        panic!("Script was not found in env '{}': {}", SCRIPT_ENV, e)
    }
}

#[marine]
pub struct Script {
    pub source_code: String,
    pub success: bool,
    pub error: String,
}

#[marine]
pub struct CID {
    pub v1_str: String,
    pub success: bool,
    pub error: String,
}

#[marine]
pub fn set_script_source_to_file(script: String) -> UnitValue {
    if !is_by_creator() {
        return UnitValue::error("Only owner of the service can set the script");
    }

    let write = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(SCRIPT_FILE)
        .map(|mut f| f.write_all(script.as_bytes()));

    match write {
        Ok(_) => UnitValue::ok(),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            UnitValue::error("Script can be set only once, and it was already set")
        }
        Err(e) => UnitValue::error(format!("Error writing script to {}: {}", SCRIPT_FILE, e)),
    }
}

#[marine]
pub fn get_script_source_from_file() -> Script {
    match std::fs::read_to_string(SCRIPT_FILE) {
        Ok(source_code) => Script {
            source_code,
            success: true,
            error: <_>::default(),
        },
        Err(e) => Script {
            source_code: <_>::default(),
            success: false,
            error: e.to_string(),
        },
    }
}

#[marine]
pub fn get_script_source_from_env() -> Script {
    match std::env::var(SCRIPT_ENV) {
        Ok(source_code) => Script {
            source_code,
            success: true,
            error: <_>::default(),
        },
        Err(e) => Script {
            source_code: <_>::default(),
            success: false,
            error: e.to_string(),
        },
    }
}

const SHA2_256: u64 = 0x12;

#[marine]
pub fn script_cid() -> CID {
    use cid::multihash::{Code, MultihashDigest};
    use cid::Cid;

    let script = get_script_source_from_file();
    if script.success {
        let digest = Code::Sha2_256.digest(script.source_code.as_bytes());
        let cid = Cid::new_v1(SHA2_256, digest);

        CID {
            v1_str: cid.to_string(),
            success: true,
            error: <_>::default(),
        }
    } else {
        CID {
            v1_str: <_>::default(),
            success: false,
            error: format!("error loading script: {}", script.error),
        }
    }
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::CallParameters;
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
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_set_script_source_to_file(spell: marine_test_env::spell::ModuleInterface) {
        assert!(
            spell
                .set_script_source_to_file("(null)".to_string())
                .success,
            "set_script_source_to_file returned false"
        );
        assert_eq!(spell.get_script_source_from_file().source_code, "(null)");
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_set_script_source_to_file_twice(spell: marine_test_env::spell::ModuleInterface) {
        assert!(
            spell
                .set_script_source_to_file("(null)".to_string())
                .success,
            "set_script_source_to_file returned false"
        );
        let second_set = spell.set_script_source_to_file("(seq (null) (null))".to_string());
        assert!(
            !second_set.success,
            "set_script_source_to_file returned true expected false"
        );
        assert_eq!(
            second_set.error,
            "Script can be set only once, and it was already set"
        );
        assert_eq!(spell.get_script_source_from_file().source_code, "(null)");
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_set_script_source_to_file_auth(spell: marine_test_env::spell::ModuleInterface) {
        let cp = CallParameters {
            init_peer_id: "folex".to_string(),
            service_creator_peer_id: "not folex".to_string(),
            service_id: "".to_string(),
            host_id: "".to_string(),
            particle_id: "".to_string(),
            tetraplets: vec![],
        };

        let set = spell.set_script_source_to_file_cp("(null)".to_string(), cp);

        assert!(!set.success, "set script succeeded while shouldn't");
        assert_eq!(set.error, "Only owner of the service can set the script");
    }

    #[marine_test(
        config_path = "../tests_artifacts/Config.toml",
        modules_dir = "../tests_artifacts"
    )]
    fn test_cid(spell: marine_test_env::spell::ModuleInterface) {
        assert!(
            spell
                .set_script_source_to_file("(null)".to_string())
                .success,
            "set_script_source_to_file returned false"
        );
        assert_eq!(
            spell.script_cid().v1_str,
            "baejbeibiotlyad7mvyqiit3ie2ljecziknctzuzmi7qtmktxmib5aiu3cq"
        );
    }
}
