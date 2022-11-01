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
