/*
 * Copyright 2024 Fluence DAO
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub const SPELL_WASM: &'static [u8] = include_bytes!("../spell-service/spell.wasm");
pub const SQLITE_WASM: &'static [u8] = include_bytes!("../spell-service/sqlite3.wasm");
pub const CONFIG: &'static [u8] = include_bytes!("../spell-service/Config.toml");

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub use build_info::PKG_VERSION as VERSION;

/// Collection of modules required to build the spell service
/// TODO: make it ordered :facepalm:
pub fn modules() -> std::collections::HashMap<&'static str, &'static [u8]> {
    maplit::hashmap! {
        "spell" => SPELL_WASM,
        "sqlite3" => SQLITE_WASM
    }
}
