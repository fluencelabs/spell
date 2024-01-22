mod roles;
mod keys;

pub use self::roles::{is_by_spell, is_by_creator};

use marine_rs_sdk::CallParameters;
use crate::auth::keys::parse_permission;
use crate::auth::roles::{Auth, authenticate};

pub fn guard_kv_write(key: &str)  -> eyre::Result<()> {
    let cp = marine_rs_sdk::get_call_parameters();
    match authenticate(&cp) {
        Auth::Spell => Ok(()),
        Auth::Other(roles) => {
            let allowed_roles = parse_permission(key);
            if !allowed_roles.is_disjoint(&roles) {
                Ok(())
            } else {
                Err(eyre::eyre!("writing to the `{key}` is forbidden for the callers with the roles: {:?}", roles))
            }
        }
    }
}


/// Check permission on writing to Spell's KV
/// *key* defines which roles can update the key
/// *call_parameters* defines which roles the caller has
pub fn is_kv_write_permitted(key: &str, call_parameters: &CallParameters) -> bool {
    match authenticate(call_parameters) {
        Auth::Spell => true,
        Auth::Other(roles) => {
            let allowed_roles = parse_permission(key);
            !allowed_roles.is_disjoint(&roles)
        }
    }
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk::CallParameters;
    use crate::auth::is_kv_write_permitted;

    const HOST_ID: &str = "host_id";
    const OTHER_HOST_ID: &str = "other_host_id";
    const SPELL_ID: &str = "spell-id";
    const OTHER_SPELL_ID: &str = "spell-2-id";
    const WORKER_ID: &str = "worker_id";
    const PARTICLE_ID: &str = "particle-id";

    const HOST_KEY: &str = "h_counter";
    const WORKER_KEY: &str = "w_message";
    const HOST_WORKER_KEY: &str = "hw_box";
    const PRIVATE_KEY: &str = "spell_settings";

    // A host that accesses a spell on itself acts as a HOST and a WORKER, so
    // keys with h_, w_ and hw_ must be available
    #[test]
    fn test_permission_host_to_host_spell() {
        let cp = CallParameters {
            init_peer_id: HOST_ID.to_string(),
            particle_id: PARTICLE_ID.to_string(),

            service_id: SPELL_ID.to_string(),
            worker_id: HOST_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(is_kv_write_permitted(HOST_KEY, &cp), "`{}` must be accessible", HOST_KEY);
        assert!(is_kv_write_permitted(HOST_WORKER_KEY, &cp), "`{}` must be accessible", HOST_WORKER_KEY);
        assert!(is_kv_write_permitted(WORKER_KEY, &cp), "`{}` must be accessible", WORKER_KEY);
        assert!(!is_kv_write_permitted(PRIVATE_KEY, &cp), "`{}` must NOT be accessible", PRIVATE_KEY);
    }

    // To self, everything must be accessible
    #[test]
    fn test_permission_host_to_self() {
        let cp = CallParameters {
            init_peer_id: HOST_ID.to_string(),
            particle_id: format!("spell_{SPELL_ID}_0"),

            service_id: SPELL_ID.to_string(),
            worker_id: HOST_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(is_kv_write_permitted(HOST_KEY, &cp), "`{}` must be accessible", HOST_KEY);
        assert!(is_kv_write_permitted(HOST_WORKER_KEY, &cp), "`{}` must be accessible", HOST_WORKER_KEY);
        assert!(is_kv_write_permitted(WORKER_KEY, &cp), "`{}` must be accessible", WORKER_KEY);
        assert!(is_kv_write_permitted(PRIVATE_KEY, &cp), "`{}` must be accessible", PRIVATE_KEY);
    }

    // A host that accesses a spell on a worker acts solely as a HOST and can modify
    #[test]
    fn test_permission_host_to_worker_spell() {
        let cp = CallParameters {
            init_peer_id: HOST_ID.to_string(),
            particle_id: "some-particle".to_string(),

            service_id: SPELL_ID.to_string(),
            worker_id: WORKER_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(is_kv_write_permitted(HOST_KEY, &cp), "`{}` must be accessible", HOST_KEY);
        assert!(is_kv_write_permitted(HOST_WORKER_KEY, &cp), "`{}` must be accessible", HOST_WORKER_KEY);
        assert!(!is_kv_write_permitted(WORKER_KEY, &cp), "`{}` must NOT be accessible", WORKER_KEY);
        assert!(!is_kv_write_permitted(PRIVATE_KEY, &cp), "`{}` must NOT be accessible", PRIVATE_KEY);

    }

    // To self, everything must be accessible
    #[test]
    fn test_permission_worker_to_self() {
        let cp = CallParameters {
            init_peer_id: WORKER_ID.to_string(),
            particle_id: format!("spell_{SPELL_ID}_0"),

            service_id: SPELL_ID.to_string(),
            worker_id: WORKER_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(is_kv_write_permitted(HOST_KEY, &cp), "`{}` must be accessible", HOST_KEY);
        assert!(is_kv_write_permitted(HOST_WORKER_KEY, &cp), "`{}` must be accessible", HOST_WORKER_KEY);
        assert!(is_kv_write_permitted(WORKER_KEY, &cp), "`{}` must be accessible", WORKER_KEY);
        assert!(is_kv_write_permitted(PRIVATE_KEY, &cp), "`{}` must be accessible", PRIVATE_KEY);

    }

    // A spell on a worker that accesses as other spell on the same worker has a WORKER permission
    // and can modify only w_ and hw_ keys
    #[test]
    fn test_permission_worker_spell_to_other_spell() {
        let cp = CallParameters {
            init_peer_id: WORKER_ID.to_string(),
            particle_id: format!("spell_{OTHER_SPELL_ID}_0"),

            service_id: SPELL_ID.to_string(),
            worker_id: WORKER_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(!is_kv_write_permitted(HOST_KEY, &cp), "`{}` must be accessible", HOST_KEY);
        assert!(is_kv_write_permitted(HOST_WORKER_KEY, &cp), "`{}` must be accessible", HOST_WORKER_KEY);
        assert!(is_kv_write_permitted(WORKER_KEY, &cp), "`{}` must be accessible", WORKER_KEY);
        assert!(!is_kv_write_permitted(PRIVATE_KEY, &cp), "`{}` must be accessible", PRIVATE_KEY);
    }

    #[test]
    fn test_permission_other() {
        let cp = CallParameters {
            init_peer_id: OTHER_HOST_ID.to_string(),
            particle_id: format!("spell_{OTHER_SPELL_ID}_0"),

            service_id: SPELL_ID.to_string(),
            worker_id: WORKER_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(!is_kv_write_permitted(HOST_KEY, &cp), "`{}` must be accessible", HOST_KEY);
        assert!(!is_kv_write_permitted(HOST_WORKER_KEY, &cp), "`{}` must be accessible", HOST_WORKER_KEY);
        assert!(!is_kv_write_permitted(WORKER_KEY, &cp), "`{}` must be accessible", WORKER_KEY);
        assert!(!is_kv_write_permitted(PRIVATE_KEY, &cp), "`{}` must be accessible", PRIVATE_KEY);

    }
}