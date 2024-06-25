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

mod keys;
mod roles;

pub use self::roles::{is_by_creator, is_by_spell};

use crate::auth::keys::parse_permission;
use crate::auth::roles::{authenticate, Role};
use marine_rs_sdk::CallParameters;

pub fn guard_kv_write(key: &str) -> eyre::Result<()> {
    let cp = marine_rs_sdk::get_call_parameters();
    check_kv_write(key, &cp)
}

pub fn is_kv_write_permitted(key: &str, call_parameters: &CallParameters) -> bool {
    check_kv_write(key, call_parameters).is_ok()
}

fn check_kv_write(key: &str, call_parameters: &CallParameters) -> eyre::Result<()> {
    match authenticate(&call_parameters) {
        Some(Role::Spell) => Ok(()),
        Some(role) => {
            let allowed_roles = parse_permission(key);
            if allowed_roles.contains(&role) {
                Ok(())
            } else {
                Err(eyre::eyre!(
                    "writing to the `{key}` is forbidden for the callers with the role {:?}",
                    role
                ))
            }
        }
        None => Err(eyre::eyre!(
            "writing to the `{key}` is forbidden for any outside caller"
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::is_kv_write_permitted;
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk::ParticleParameters;

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
            particle: ParticleParameters {
                init_peer_id: HOST_ID.to_string(),
                id: PARTICLE_ID.to_string(),
                ..<_>::default()
            },

            service_id: SPELL_ID.to_string(),
            worker_id: HOST_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(
            is_kv_write_permitted(HOST_KEY, &cp),
            "`{}` must be accessible",
            HOST_KEY
        );
        assert!(
            is_kv_write_permitted(HOST_WORKER_KEY, &cp),
            "`{}` must be accessible",
            HOST_WORKER_KEY
        );
        // it's our choice that a host spell tryng to access other host spells will be considered a host not a worker
        assert!(
            !is_kv_write_permitted(WORKER_KEY, &cp),
            "`{}` must NOT be accessible",
            WORKER_KEY
        );
        assert!(
            !is_kv_write_permitted(PRIVATE_KEY, &cp),
            "`{}` must NOT be accessible",
            PRIVATE_KEY
        );
    }

    // To self, everything must be accessible
    #[test]
    fn test_permission_host_to_self() {
        let cp = CallParameters {
            particle: ParticleParameters {
                init_peer_id: HOST_ID.to_string(),
                id: format!("spell_{SPELL_ID}_0"),
                ..<_>::default()
            },

            service_id: SPELL_ID.to_string(),
            worker_id: HOST_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(
            is_kv_write_permitted(HOST_KEY, &cp),
            "`{}` must be accessible",
            HOST_KEY
        );
        assert!(
            is_kv_write_permitted(HOST_WORKER_KEY, &cp),
            "`{}` must be accessible",
            HOST_WORKER_KEY
        );
        assert!(
            is_kv_write_permitted(WORKER_KEY, &cp),
            "`{}` must be accessible",
            WORKER_KEY
        );
        assert!(
            is_kv_write_permitted(PRIVATE_KEY, &cp),
            "`{}` must be accessible",
            PRIVATE_KEY
        );
    }

    // A host that accesses a spell on a worker acts solely as a HOST and can modify
    #[test]
    fn test_permission_host_to_worker_spell() {
        let cp = CallParameters {
            particle: ParticleParameters {
                init_peer_id: HOST_ID.to_string(),
                id: "some-particle".to_string(),
                ..<_>::default()
            },

            service_id: SPELL_ID.to_string(),
            worker_id: WORKER_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(
            is_kv_write_permitted(HOST_KEY, &cp),
            "`{}` must be accessible",
            HOST_KEY
        );
        assert!(
            is_kv_write_permitted(HOST_WORKER_KEY, &cp),
            "`{}` must be accessible",
            HOST_WORKER_KEY
        );
        assert!(
            !is_kv_write_permitted(WORKER_KEY, &cp),
            "`{}` must NOT be accessible",
            WORKER_KEY
        );
        assert!(
            !is_kv_write_permitted(PRIVATE_KEY, &cp),
            "`{}` must NOT be accessible",
            PRIVATE_KEY
        );
    }

    // To self, everything must be accessible
    #[test]
    fn test_permission_worker_to_self() {
        let cp = CallParameters {
            particle: ParticleParameters {
                init_peer_id: WORKER_ID.to_string(),
                id: format!("spell_{SPELL_ID}_0"),
                ..<_>::default()
            },

            service_id: SPELL_ID.to_string(),
            worker_id: WORKER_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(
            is_kv_write_permitted(HOST_KEY, &cp),
            "`{}` must be accessible",
            HOST_KEY
        );
        assert!(
            is_kv_write_permitted(HOST_WORKER_KEY, &cp),
            "`{}` must be accessible",
            HOST_WORKER_KEY
        );
        assert!(
            is_kv_write_permitted(WORKER_KEY, &cp),
            "`{}` must be accessible",
            WORKER_KEY
        );
        assert!(
            is_kv_write_permitted(PRIVATE_KEY, &cp),
            "`{}` must be accessible",
            PRIVATE_KEY
        );
    }

    // A spell on a worker that accesses as other spell on the same worker has a WORKER permission
    // and can modify only w_ and hw_ keys
    #[test]
    fn test_permission_worker_spell_to_other_spell() {
        let cp = CallParameters {
            particle: ParticleParameters {
                init_peer_id: WORKER_ID.to_string(),
                id: format!("spell_{OTHER_SPELL_ID}_0"),
                ..<_>::default()
            },

            service_id: SPELL_ID.to_string(),
            worker_id: WORKER_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(
            !is_kv_write_permitted(HOST_KEY, &cp),
            "`{}` must be NOT accessible",
            HOST_KEY
        );
        assert!(
            is_kv_write_permitted(HOST_WORKER_KEY, &cp),
            "`{}` must be accessible",
            HOST_WORKER_KEY
        );
        assert!(
            is_kv_write_permitted(WORKER_KEY, &cp),
            "`{}` must be accessible",
            WORKER_KEY
        );
        assert!(
            !is_kv_write_permitted(PRIVATE_KEY, &cp),
            "`{}` must be NOT accessible",
            PRIVATE_KEY
        );
    }

    #[test]
    fn test_permission_other() {
        let cp = CallParameters {
            particle: ParticleParameters {
                init_peer_id: OTHER_HOST_ID.to_string(),
                id: format!("spell_{OTHER_SPELL_ID}_0"),
                ..<_>::default()
            },

            service_id: SPELL_ID.to_string(),
            worker_id: WORKER_ID.to_string(),
            host_id: HOST_ID.to_string(),
            service_creator_peer_id: HOST_ID.to_string(),

            tetraplets: vec![],
        };

        assert!(
            !is_kv_write_permitted(HOST_KEY, &cp),
            "`{}` must be NOT accessible",
            HOST_KEY
        );
        assert!(
            !is_kv_write_permitted(HOST_WORKER_KEY, &cp),
            "`{}` must be NOT accessible",
            HOST_WORKER_KEY
        );
        assert!(
            !is_kv_write_permitted(WORKER_KEY, &cp),
            "`{}` must be NOT accessible",
            WORKER_KEY
        );
        assert!(
            !is_kv_write_permitted(PRIVATE_KEY, &cp),
            "`{}` must be NOT accessible",
            PRIVATE_KEY
        );
    }
}
