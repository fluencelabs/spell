/*
 * Aqua Spell Service
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;

use eyre::Context;
use marine_rs_sdk::marine;
use serde_json::Value as JValue;

use crate::auth::guard_kv_write;
use fluence_spell_dtos::value::UnitValue;

use crate::kv::primitive::store_string;

#[marine]
/// Save all fields of the passed object to KV as strings.
/// NOTE: this function is not recursive. It takes only first-level fields.
pub fn set_json_fields(json: &str) -> UnitValue {
    let result: eyre::Result<()> = try {
        let fields: HashMap<String, JValue> =
            serde_json::from_str(json).context("passed string must represent a JSON object")?;
        // TODO: should it be all in a single transaction?
        for (key, value) in fields {
            guard_kv_write(&key)?;
            store_string(&key, value.to_string())
                .context(format!("set string for field '{}' failed", key))?
        }
    };

    result.into()
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk::ParticleParameters;
    use marine_rs_sdk_test::marine_test;
    use serde_json::json;

    const DB_FILE: &str = "./tests_artifacts/spell.sqlite";

    #[ctor::ctor]
    /// usage of 'ctor' makes this function run only once
    fn before_all_tests() {
        std::fs::remove_file(DB_FILE).ok();
    }

    /// after_each macro copy-pastes this function into every test
    fn after_each() {
        std::fs::remove_file(DB_FILE).ok();
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_json(spell: marine_test_env::spell::ModuleInterface) {
        let json = json!({
            "a": 1,
            "b": {"foo": "bar"},
            "c": "string",
            "d": [],
            "e": ["1", "2", "3"],
            "f": [1, "2", "three", [4], ["five"], [{"six": 7}]]
        });
        let set = spell.set_json_fields_cp(json.to_string(), spell_call_params());
        assert!(set.success, "set_json_fields failed: {}", set.error);

        let values = json.as_object().unwrap();
        for (k, v) in values {
            let get = spell.get_string(k.to_string());
            assert!(get.success, "get_string {} failed: {}", k, get.error);
            assert_eq!(get.value, v.to_string());
        }
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_json_host(spell: marine_test_env::spell::ModuleInterface) {
        let json = json!({
            "hw_a": 1,
            "h_b": {"foo": "bar"},
        });
        let set = spell.set_json_fields_cp(json.to_string(), host_call_params());
        assert!(set.success, "set_json_fields failed: {}", set.error);

        let values = json.as_object().unwrap();
        for (k, v) in values {
            let get = spell.get_string(k.to_string());
            assert!(get.success, "get_string {} failed: {}", k, get.error);
            assert_eq!(get.value, v.to_string());
        }
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_json_worker(spell: marine_test_env::spell::ModuleInterface) {
        let json = json!({
            "hw_a": 1,
            "w_b": {"foo": "bar"},
        });
        let set = spell.set_json_fields_cp(json.to_string(), worker_call_params());
        assert!(set.success, "set_json_fields failed: {}", set.error);

        let values = json.as_object().unwrap();
        for (k, v) in values {
            let get = spell.get_string(k.to_string());
            assert!(get.success, "get_string {} failed: {}", k, get.error);
            assert_eq!(get.value, v.to_string());
        }
    }

    #[marine_test(config_path = "../../tests_artifacts/Config.toml")]
    fn test_json_other(spell: marine_test_env::spell::ModuleInterface) {
        let json = json!({
            "hw_a": 1,
            "w_b": {"foo": "bar"},
        });
        let set = spell.set_json_fields_cp(json.to_string(), other_call_params());
        assert!(!set.success, "set_json_fields must fail");
    }

    fn spell_call_params() -> CallParameters {
        CallParameters {
            particle: ParticleParameters {
                init_peer_id: "worker-id".to_string(),
                id: "spell_spell-id_0".to_string(),
                ..<_>::default()
            },
            service_creator_peer_id: "worker-id".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }

    fn host_call_params() -> CallParameters {
        CallParameters {
            particle: ParticleParameters {
                init_peer_id: "host-id".to_string(),
                id: "some-particle".to_string(),
                ..<_>::default()
            },
            service_creator_peer_id: "worker-id".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }

    fn worker_call_params() -> CallParameters {
        CallParameters {
            particle: ParticleParameters {
                init_peer_id: "worker-id".to_string(),
                id: "some-particle".to_string(),
                ..<_>::default()
            },
            service_creator_peer_id: "worker-id".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }

    fn other_call_params() -> CallParameters {
        CallParameters {
            particle: ParticleParameters {
                init_peer_id: "other-worker-id".to_string(),
                id: "some-particle".to_string(),
                ..<_>::default()
            },
            service_creator_peer_id: "worker-id".to_string(),
            service_id: "spell-id".to_string(),
            worker_id: "worker-id".to_string(),
            host_id: "host-id".to_string(),
            tetraplets: vec![],
        }
    }
}
