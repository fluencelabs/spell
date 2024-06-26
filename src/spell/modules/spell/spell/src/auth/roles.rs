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

use marine_rs_sdk::{get_call_parameters, CallParameters};

/// In our Spell KV Security model we have several roles:
/// - Host -- the call is performed from the name of the host
/// - Worker -- the call is performed from the name of the worker on which the target spell is running
/// - Spell -- the call is performed by the spell itself from the name of the worker the spell is installed on
///

/// A role that a caller can have
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Role {
    // The call is performed from the name of the spell itself
    Spell,
    // The call is performed from the name of the peer that hosts the spell
    Host,
    // The call is performed from the name of the worker that owns the spell
    Worker,
}

pub fn authenticate(call_parameters: &CallParameters) -> Option<Role> {
    if is_spell(&call_parameters) {
        return Some(Role::Spell);
    }
    if is_by_host(call_parameters) {
        return Some(Role::Host);
    }
    if is_by_worker(call_parameters) {
        return Some(Role::Worker);
    }
    return None;
}

/// returns true if function is called by the service creator
pub fn is_by_creator() -> bool {
    let call_parameters = get_call_parameters();

    call_parameters.particle.init_peer_id == call_parameters.service_creator_peer_id
}

/// Used to protect methods that should be called only by the spell itself.
/// true if particle id has a form of `spell_<spell_id>_<counter>`
/// and `is_by_creator` returns true (because anyone can set any particle id)
pub fn is_by_spell(call_parameters: &CallParameters) -> bool {
    let particle_id: &str = &call_parameters.particle.id;
    if particle_id.starts_with("spell") {
        if let Some(spell_id) = particle_id.split("_").skip(1).next() {
            return spell_id == call_parameters.service_id.as_str() && is_by_creator();
        }
    }

    return false;
}
/// Check if the call is performed by the host
fn is_by_host(call_parameters: &CallParameters) -> bool {
    call_parameters.particle.init_peer_id == call_parameters.host_id
}

/// Check if the call is performed by the worker
/// A service creator of the spell must be it's worker, so
/// if the call was initiated by a peer the same as the spell service creator
/// we may safely conclude, it's a spell from the same worker
fn is_by_worker(call_parameters: &CallParameters) -> bool {
    call_parameters.particle.init_peer_id == call_parameters.worker_id
}

/// Check if the call is performed by the spell itself
/// To check it we need to look at:
/// - if the call was produced by the worker
/// - if the particle_id contains the spell id of the calling spell
/// Here we rely on the convention that particle_id of a spell contains its spell_id.
/// We also rely on the fact that particle_id can't forged since it's backed up by particle signatures
fn is_spell(call_parameters: &CallParameters) -> bool {
    is_spell_particle(call_parameters) && is_by_worker(call_parameters)
}

fn is_spell_particle(call_parameters: &CallParameters) -> bool {
    let particle_id: &str = &call_parameters.particle.id;
    if particle_id.starts_with("spell") {
        if let Some(spell_id) = particle_id.split("_").skip(1).next() {
            return spell_id == call_parameters.service_id.as_str();
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::auth::{authenticate, roles::Role};
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk::ParticleParameters;

    #[test]
    fn test_request_from_host_spell_to_self() {
        let cp_from_host = CallParameters {
            particle: ParticleParameters {
                init_peer_id: "host_id".to_string(),
                id: "spell_spell-id_0".to_string(),
                ..<_>::default()
            },
            service_id: "spell-id".to_string(),
            service_creator_peer_id: "host_id".to_string(),
            worker_id: "host_id".to_string(),
            host_id: "host_id".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Some(Role::Spell), authenticate(&cp_from_host));
    }

    #[test]
    fn test_request_from_host_spell_to_hosts_other_spell() {
        let cp_from_host = CallParameters {
            particle: ParticleParameters {
                init_peer_id: "host_id".to_string(),
                id: "spell_spell-2-id_0".to_string(),
                ..<_>::default()
            },
            service_id: "spell-id".to_string(),
            service_creator_peer_id: "host_id".to_string(),
            worker_id: "host_id".to_string(),
            host_id: "host_id".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Some(Role::Host), authenticate(&cp_from_host));
    }

    #[test]
    fn test_request_from_host_to_worker_spell() {
        let cp_from_host = CallParameters {
            particle: ParticleParameters {
                init_peer_id: "host_id".to_string(),
                id: "spell_spell-2-id_0".to_string(),
                ..<_>::default()
            },
            service_id: "spell-id".to_string(),
            service_creator_peer_id: "worker_id".to_string(),
            worker_id: "worker_id".to_string(),
            host_id: "host_id".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Some(Role::Host), authenticate(&cp_from_host));
    }

    #[test]
    fn test_request_from_worker_spell_to_self() {
        let cp_from_host = CallParameters {
            particle: ParticleParameters {
                init_peer_id: "worker_id".to_string(),
                id: "spell_spell-1-id_0".to_string(),
                ..<_>::default()
            },
            service_id: "spell-1-id".to_string(),
            service_creator_peer_id: "worker_id".to_string(),
            worker_id: "worker_id".to_string(),
            host_id: "host_id".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Some(Role::Spell), authenticate(&cp_from_host));
    }

    #[test]
    fn test_request_from_worker_spell_to_other_spell_on_the_worker() {
        let cp_from_host = CallParameters {
            particle: ParticleParameters {
                init_peer_id: "worker_id".to_string(),
                id: "spell_spell-2-id_0".to_string(),
                ..<_>::default()
            },
            service_id: "spell-1-id".to_string(),
            service_creator_peer_id: "worker_id".to_string(),
            worker_id: "worker_id".to_string(),
            host_id: "host_id".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Some(Role::Worker), authenticate(&cp_from_host));
    }

    #[test]
    fn test_request_from_other_worker() {
        let cp_from_host = CallParameters {
            particle: ParticleParameters {
                init_peer_id: "worker_2_id".to_string(),
                id: "spell_spell-2-id_0".to_string(),
                ..<_>::default()
            },
            service_id: "spell-1-id".to_string(),
            service_creator_peer_id: "worker_id".to_string(),
            worker_id: "worker_id".to_string(),
            host_id: "host_id".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(None, authenticate(&cp_from_host));
    }
}
