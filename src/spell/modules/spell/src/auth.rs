/// returns true if function is called by the service creator
pub fn is_by_creator() -> bool {
    let call_parameters = marine_rs_sdk::get_call_parameters();

    call_parameters.init_peer_id == call_parameters.service_creator_peer_id
}
