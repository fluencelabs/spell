use std::collections::HashMap;

use eyre::Context;
use marine_rs_sdk::marine;
use serde_json::Value as JValue;

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
        for (k, v) in fields {
            store_string(&k, v.to_string())
                .context(format!("set string for field '{}' failed", k))?
        }
    };

    result.into()
}

#[test_env_helpers::after_each]
#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;
    use serde_json::json;

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
        let set = spell.set_json_fields(json.to_string());
        assert!(set.success, "set_json_fields failed: {}", set.error);

        let values = json.as_object().unwrap();
        for (k, v) in values {
            let get = spell.get_string(k.to_string());
            assert!(get.success, "get_string {} failed: {}", k, get.error);
            assert_eq!(get.value, v.to_string());
        }
    }
}
