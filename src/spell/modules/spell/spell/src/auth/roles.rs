use std::collections::HashSet;
use std::fmt::Display;
use marine_rs_sdk::{get_call_parameters, CallParameters};

/// In our Spell KV Security model we have several roles:
/// - Host -- the call is performed from the name of the host
/// - Worker -- the call is performed from the name of the worker on which the target spell is running
/// - Spell -- the call is performed by the spell itself from the name of the worker the spell is installed on
///
///


/// A role that a non-owner can have
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Role {
    // The call is performed from the name of the peer that hosts the spell
    Host,
    // The call is performed from the name of the worker that owns the spell
    Worker,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = match self {
            Role::Host => "host",
            Role::Worker => "worker",
        };
        write!(f, "{}", role)
    }
}

/// A set of rights the caller has
#[derive(Debug, Eq, PartialEq)]
pub enum Auth {
    // The caller is the spell itself that has all the rights to do anything
    Spell,
    // The caller is some other entity and can have several or none roles assigned to it
    // and requires detailed permission checking
    Other(HashSet<Role>)
}

pub fn authenticate(call_parameters: &CallParameters) -> Auth {
    if is_spell(&call_parameters) {
        return Auth::Spell;
    }
    let mut entity = HashSet::new();
    if is_worker(call_parameters) {
        entity.insert(Role::Worker);
    }
    if is_host(call_parameters) {
        entity.insert(Role::Host);
    }
    Auth::Other(entity)
}


/// returns true if function is called by the service creator
pub fn is_by_creator() -> bool {
    let call_parameters = get_call_parameters();

    call_parameters.init_peer_id == call_parameters.service_creator_peer_id
}

/// Used to protect methods that should be called only by the spell itself.
/// true if particle id has a form of `spell_<spell_id>_<counter>`
/// and `is_by_creator` returns true (because anyone can set any particle id)
pub fn is_by_spell(call_parameters: &CallParameters) -> bool {
    let particle_id: &str = &call_parameters.particle_id;
    if particle_id.starts_with("spell") {
        if let Some(spell_id) = particle_id.split("_").skip(1).next() {
            return spell_id == call_parameters.service_id.as_str() && is_by_creator();
        }
    }

    return false;
}
/// Check if the call is performed by the host
fn is_host(call_parameters: &CallParameters) -> bool {
    call_parameters.init_peer_id == call_parameters.host_id
}

/// Check if the call is performed by the worker
/// A service creator of the spell must be it's worker, so
/// if the call was initiated by a peer the same as the spell service creator
/// we may safely conclude, it's a spell from the same worker
fn is_worker(call_parameters: &CallParameters) -> bool {
   call_parameters.init_peer_id == call_parameters.worker_id
}

/// Check if the call is performed by the spell itself
/// To check it we need to look at:
/// - if the call was produced by the worker
/// - if the particle_id contains the spell id of the calling spell
/// Here we rely on the convention that particle_id of a spell contains its spell_id.
/// We also rely on the fact that particle_id can't forged since it's backed up by particle signatures
fn is_spell(call_parameters: &CallParameters) -> bool {
    is_spell_particle(call_parameters) && is_worker(call_parameters)
}

fn is_spell_particle(call_parameters: &CallParameters) -> bool {
    let particle_id: &str = &call_parameters.particle_id;
    if particle_id.starts_with("spell") {
        if let Some(spell_id) = particle_id.split("_").skip(1).next() {
            return spell_id == call_parameters.service_id.as_str()
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use marine_rs_sdk::CallParameters;
    use crate::auth::{authenticate, Role, Auth};

    #[test]
    fn test_request_from_host_spell_to_self() {
        let cp_from_host = CallParameters {
            init_peer_id: "host_id".to_string(),
            service_id: "spell-id".to_string(),
            service_creator_peer_id: "host_id".to_string(),
            worker_id: "host_id".to_string(),
            host_id: "host_id".to_string(),
            particle_id: "spell_spell-id_0".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Auth::Spell, authenticate(&cp_from_host));
    }


    #[test]
    fn test_request_from_host_spell_to_hosts_other_spell() {
        let cp_from_host = CallParameters {
            init_peer_id: "host_id".to_string(),
            service_id: "spell-id".to_string(),
            service_creator_peer_id: "host_id".to_string(),
            worker_id: "host_id".to_string(),
            host_id: "host_id".to_string(),
            particle_id: "spell_spell-2-id_0".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Auth::Other(HashSet::from([Role::Host, Role::Worker])), authenticate(&cp_from_host));
    }

    #[test]
    fn test_request_from_host_to_worker_spell() {
        let cp_from_host = CallParameters {
            init_peer_id: "host_id".to_string(),
            service_id: "spell-id".to_string(),
            service_creator_peer_id: "worker_id".to_string(),
            worker_id: "worker_id".to_string(),
            host_id: "host_id".to_string(),
            particle_id: "spell_spell-2-id_0".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Auth::Other(HashSet::from([Role::Host])), authenticate(&cp_from_host));
    }

    #[test]
    fn test_request_from_worker_spell_to_self() {
        let cp_from_host = CallParameters {
            init_peer_id: "worker_id".to_string(),
            service_id: "spell-1-id".to_string(),
            service_creator_peer_id: "worker_id".to_string(),
            worker_id: "worker_id".to_string(),
            host_id: "host_id".to_string(),
            particle_id: "spell_spell-1-id_0".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Auth::Spell, authenticate(&cp_from_host));
    }

    #[test]
    fn test_request_from_worker_spell_to_other_spell_on_the_worker() {
        let cp_from_host = CallParameters {
            init_peer_id: "worker_id".to_string(),
            service_id: "spell-1-id".to_string(),
            service_creator_peer_id: "worker_id".to_string(),
            worker_id: "worker_id".to_string(),
            host_id: "host_id".to_string(),
            particle_id: "spell_spell-2-id_0".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Auth::Other(HashSet::from([Role::Worker])), authenticate(&cp_from_host));
    }

    #[test]
    fn test_request_from_other_worker() {
        let cp_from_host = CallParameters {
            init_peer_id: "worker_2_id".to_string(),
            service_id: "spell-1-id".to_string(),
            service_creator_peer_id: "worker_id".to_string(),
            worker_id: "worker_id".to_string(),
            host_id: "host_id".to_string(),
            particle_id: "spell_spell-2-id_0".to_string(),
            tetraplets: vec![],
        };
        assert_eq!(Auth::Other(HashSet::new()), authenticate(&cp_from_host));
    }
}