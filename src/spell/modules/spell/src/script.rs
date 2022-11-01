use marine_rs_sdk::marine;

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
    pub error: String
}

#[marine]
pub struct CID {
    pub v1_str: String,
    pub success: bool,
    pub error: String
}

#[marine]
pub fn set_script_source_to_file(script: String) -> bool {
    std::fs::write(SCRIPT_FILE, script).is_ok()
}

#[marine]
pub fn get_script_source_from_file() -> Script {
    match std::fs::read_to_string(SCRIPT_FILE) {
        Ok(source_code) => Script { source_code, success: true, error: <_>::default() },
        Err(e) => Script { source_code: <_>::default(), success: false, error: e.to_string() }
    }
}

#[marine]
pub fn get_script_source_from_env() -> Script {
    match std::env::var(SCRIPT_ENV) {
        Ok(source_code) => Script { source_code, success: true, error: <_>::default() },
        Err(e) => Script { source_code: <_>::default(), success: false, error: e.to_string() }
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

        CID { v1_str: cid.to_string(), success: true, error: <_>::default() }
    } else {
        CID { v1_str: <_>::default(), success: false, error: format!("error loading script: {}", script.error) }
    }
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../tests_artifacts/Config.toml", modules_dir = "../tests_artifacts")]
    fn test_set_script_source_to_file(spell: marine_test_env::spell::ModuleInterface) {
        assert!(spell.set_script_source_to_file("(null)".to_string()), "set_script_source_to_file returned false");
        assert_eq!(spell.get_script_source_from_file().source_code, "(null)");
    }

    #[marine_test(config_path = "../tests_artifacts/Config.toml", modules_dir = "../tests_artifacts")]
    fn test_cid(spell: marine_test_env::spell::ModuleInterface) {
        assert!(spell.set_script_source_to_file("(null)".to_string()), "set_script_source_to_file returned false");
        assert_eq!(spell.script_cid().v1_str, "baejbeibiotlyad7mvyqiit3ie2ljecziknctzuzmi7qtmktxmib5aiu3cq");
    }
}