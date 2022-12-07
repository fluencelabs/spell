pub const SPELL_WASM: &'static [u8] = include_bytes!("../spell-service/spell.wasm");
pub const SQLITE_WASM: &'static [u8] = include_bytes!("../spell-service/sqlite3.wasm");
pub const CONFIG: &'static [u8] = include_bytes!("../spell-service/Config.toml");

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub use build_info::PKG_VERSION as VERSION;

pub fn modules() -> std::collections::HashMap<&'static str, &'static [u8]> {
    maplit::hashmap! {
        "spell" => SPELL_WASM,
        "sqlite3" => SQLITE_WASM
    }
}
