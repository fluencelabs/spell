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

## Test builtin spell functions: `install`, `remove`, `list`, `update_trigger_config`

### Spell installation

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

### Spell removal

Requires an additional service (or a spell that do nothing) that could store
info from the spell we're testing.

_Input script_: spell affects another aux service.

_Input config_: any type that runs immediately

_Input state_: empty

Checks after removal:

1. the spell is unavailable via its `spell_id`
2. no action are executed by the spell: the other service isn't affected by the
   spell anymore

_Input script_: script do nothing

_Input config_: delayed config that won't run before execution

_Input state_: empty

Checks after removal:

1. the spell is unavailable via its `spell_id`
2. the spell stopped execution: the aux service isn't affected by the spell

_Input_: any correct input

Checks:

1. remove a spell via `srv.remove` is failing
2. `spell.remove` can't remove non-spell services

### List

Note that `list` doesn't show running/stopped spells, just the installed ones.

Checks:

1. After installing the spell, its id is in the list.
2. After removing the spell, its id isn't in the list.

### Trigger config updates

_Input script_: do nothing

_Input state_: empty

Plan:

1. Set one-shot config. Check that it's executed.
2. Wait a bit and check that the counter isn't changed.
3. Set another one-shot config. Check that it's executed.

## Spell execution

## Test trigger config

Use aux service like in the removal tests to detect if the spell stopped the
execution

_Input script_: script that affects aux service

_Input state_: empty

_Input config_: empty

Checks:

1. Wait and check `"count"`.

### Timer config

_Input script_: script that affects aux service

_Input state_: empty

_Input config_: one-shot that runs immediately Checks:

1. Check that the counter equals 1. Wait several seconds, and then check again.
2. Check the aux service

_Input config_: periodic, 1 sec, that runs immediately

Checks:

1. Wait several seconds, and check that the counter is not zero and it's not
   more than waited amount of seconds + some delta

_Input script_: any

_Input state_: empty

_Input config_: periodic, 1 sec, which ends in several seconds

Checks:

1. Wait until `end_sec`, check that the counter is not zero and it's not more
   than waited amount of seconds + some delta,
2. Check that the spell isn't executed anymore via the aux service

### Connection pool trigger config

_Input script_: any _Input state_: empty

TODO: how to control connections and disconnections? Via some JS-code?

_Input config_: react on connect

Checks:

1. Check `"count"`

_Input config_: react on disconnect

Checks:

1. Check `"count"`

### Mixed configs

_Input script_: any

_Input state_: any

_Input config_: timer config that starts in the future + connection trigger
Checks:

1. Counter is non-zero.

TODO: more complex tests when the spell will be receiving the trigger info

### Bad configs

Check that all incorrect configs are rejected. Right now only ClockConfig can be
bad.

Incorrect configs:

- `end_sec < start_sec`
- `end_sec` is in the past
- `period_sec` is very big

## Error handling

TODO: what expected?

## Permissions

When they are implemented.