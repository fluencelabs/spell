use marine_rs_sdk::{get_call_parameters, CallParameters};
// something

/// returns true if function is called by the service creator
pub fn is_by_creator() -> bool {
    let call_parameters = get_call_parameters();

    call_parameters.init_peer_id == call_parameters.service_creator_peer_id
}

/// returns true if call was made from the associated spell script
pub fn is_by_spell(call_parameters: &CallParameters) -> bool {
    let particle_id: &str = &call_parameters.particle_id;
    if particle_id.starts_with("spell") {
        if let Some(spell_id) = particle_id.split("_").skip(1).next() {
            return spell_id == call_parameters.service_id.as_str();
        }
    }

    return false;
}
