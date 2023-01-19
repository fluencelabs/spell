#Spells Integration Tests

## How to run

- `npm i`
- `pip3 install -r requirements.txt`
- `pytest -n auto`

# Test Plan

## Quick check

_Input script_: increment a value from zero

```air
(seq
    (seq
        (call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id)
        (call %init_peer_id% ("getDataSrv" "value") [] value)
     )
     (seq
        (call %init_peer_id% ("math" "add") [value 1] result)
        (call %init_peer_id% ("callbackSrv" "response") ["value" value])
     )
)
```

_Input config_: run indefinitely every second

```json
{
  "clock": { "start_sec": 1, "end_sec": 0, "period_sec": 1 },
  "connections": { "connect": false, "disconnect": false },
  "blockchain": { "start_block": 0, "end_block": 0 }
}
```

_Input state_: empty

Plan:

1. Install a spell
2. Run it for several seconds
3. Update the config to stop the execution (for that send an empty config)
4. Check that the incremented value is equal to `"count"`
5. Remove the spell


## Testing rust-peer Spell API

### Test builtin spell functions: `install`, `remove`, `list`, `update_trigger_config`

#### Spell installation

_Input script_: script does nothing

_Input config_: oneshot config that runs immediately

_Input state_: empty

Checks:

1. `spell.install` on correct arguments must return `spell_id`
2. original trigger config that was passed to `spell.install` must match
   `spell_id.get_trigger_config`: the config must be saved to the corresponding
   spell service on installtion
3. original script that was passed to `spell.install` must match
   `spell_id.get_script_source_from_file`: the script must be saved to the
   corresponding spell service on installation
4. `spell_id.get_script("count")` should be non-zero: the spell must be
   subcribed to execution and run. NOTE: we may need to wait until the spell is
   executed

#### Spell removal

Requires an additional service (or a spell that do nothing) that could store
info from the spell we're testing.

**Remove running**

*Input script*: spell affects another aux service.

_Input config_: any type that runs immediately

_Input state_: empty

Checks after removal:

1. the spell is unavailable via its `spell_id`
2. no action are executed by the spell: the other service isn't affected by the
   spell anymore

**Remove never run**

*Input script*: script do nothing

*Input config*: empty config

_Input state_: empty

Checks after removal:

1. the spell is unavailable via its `spell_id`
2. the spell stopped execution: the aux service isn't affected by the spell

**Remove stopped**

*Input script*: script do nothing

*Input config*: any type that runs immediately

*Input state*: empty

Checks after removal:
1. Stop the spell
2. the spell is unavailable via its `spell_id`
3. the spell stopped execution: the aux service isn't affected by the spell

**Check API**

*Input*: any correct input

Checks:

1. remove a spell via `srv.remove` is failing
2. `spell.remove` can't remove non-spell services (TODO: need to implement good way of obtaining a simple service first)

#### List

Note that `list` doesn't show running/stopped spells, just the installed ones.

Checks:

1. After installing the spell, its id is in the list.
2. After removing the spell, its id isn't in the list.

#### Trigger config updates

**Basic functionallity**

_Input script_: do nothing

*Input state*: empty

Plan:
1. Check that trigger mailbox is empty and its counter is empty.
2. Set oneshot config.
3. Check that trigger mailbox contains one timer trigger and its counter is one.

**Permissions**

*Input script*: do nothing

_Input config_: empty

*Input state*: empty

1. Try to update the spell with different sk and fail.

#### Configs

**Timer config: periodic**

_Input config_: periodic, 1 sec, that runs immediately

Checks:

**Timer config: periodic and end_sec**

*Input script*: any

_Input script_: any

_Input state_: empty

_Input config_: periodic, 1 sec, which ends in several seconds

Checks:

1. Wait until `end_sec`, check that the counter is not zero and it's not more
   than waited amount of seconds + some delta,
2. Check that the spell isn't executed anymore via the aux service

#### Connection pool trigger config

_Input script_: any _Input state_: empty

TODO: how to control connections and disconnections? Via some JS-code?

_Input config_: react on connect

Checks:
1. Check `"count"` and mailbox

_Input config_: react on disconnect

Checks:
1. Check `"count"` and mailbox

#### Mixed configs

_Input script_: any

_Input state_: any

_Input config_: timer config that starts in the future + connection trigger
Checks:

1. Counter is non-zero.

TODO: more complex tests when the spell will be receiving the trigger info

#### Bad configs

Check that all incorrect configs are rejected. Right now only ClockConfig can be
bad.

Incorrect configs:

- `end_sec < start_sec`
- `end_sec` is in the past
- `period_sec` is very big

### Permissions

When they are implemented.

## Testing Spell Service API

WIP
