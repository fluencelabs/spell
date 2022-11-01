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
    source_code: String,
    success: bool,
    error: String
}

#[marine]
pub fn setScriptSourceToFile(script: String) -> bool {
    std::fs::write(SCRIPT_FILE, script).ok()
}

#[marine]
pub fn getScriptSourceFromFile() -> Script {
    match std::fs::read_to_string(SCRIPT_FILE) {
        Ok(script) => Script { source_code, success: true, error: <_>::default() },
        Err(e) => Script { source_code: <_>::default(), success: false, error: e.to_string() }
    }
}

#[marine]
pub fn getScriptSourceFromEnv() -> Script {
    match std::env::var(SCRIPT_ENV) {
        Ok(source_code) => Script { source_code, success: true, error: <_>::default() },
        Err(e) => Script { source_code: <_>::default(), success: false, error: e.to_string() }
    }
}
