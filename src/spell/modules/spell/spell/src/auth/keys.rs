use crate::auth::roles::Role;
use std::collections::HashSet;

/// Keys parsing module to determine the available rights.
///
/// Format: `{special_prefix}_{key}`
/// Where `special_prefix` is a string with encoded rights and `key` is a user-defined string with
/// any allowed symbols.
///
/// `special_prefix` can be either:
/// - `h` -- the key can be modified by the *host*
/// - `w` -- the key can be modified by a *worker* (the worker spell or other spells on the same worker)
/// - `hw` -- the key can be modified both by a *host* or a *worker*.
/// Keys without a `special_prefix` can be modified only by a spell itself.
///
/// Examples:
/// - `h_worker_def_cid` can be modified by the host
/// - `w_fellow_spell_last_ping_timestamp` can be modified by a worker
/// - `hw_artificial_mailbox` can be modified both by the host and a worker
/// - `wh_count` can be modified by a spell alone despite the `wh` string containing all required symbols
/// - `worker_settings` can be modified only by a spell
///
/// Note that everyone have READ rights
///
pub fn parse_permission(key: &str) -> HashSet<Role> {
    match key.split_once('_') {
        Some(("h", _)) => HashSet::from([Role::Host]),
        Some(("w", _)) => HashSet::from([Role::Worker]),
        Some(("hw", _)) => HashSet::from([Role::Host, Role::Worker]),
        _ => HashSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::keys::parse_permission;
    use crate::auth::roles::Role;
    use std::collections::HashSet;

    #[test]
    fn test_key_parse() {
        let data = vec![
            ("h_worker_def_cid", HashSet::from([Role::Host])),
            (
                "w_fellow_spell_last_ping_timestamp",
                HashSet::from([Role::Worker]),
            ),
            (
                "hw_artificial_mailbox",
                HashSet::from([Role::Host, Role::Worker]),
            ),
            ("wh_count", HashSet::new()),
            ("hword_count", HashSet::new()),
            ("worker_settings", HashSet::new()),
        ];
        for (key, result) in data {
            assert_eq!(result, parse_permission(key), "parsing {key}");
        }
    }
}
