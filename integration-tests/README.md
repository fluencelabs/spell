# Spells Integration Tests

## How to run

- `npm i`
- `pip3 install -r requirements.txt`
- `pytest -n auto`

## Test Plan

### Quick check

_Input script_: increment a value from zero

```aqua
data IncState:
	value: i64

func inc_value(value: string) -> string:
    value_real <- JsonNum.parse(value)
    result <- Math.add(value_real, 1)
	obj = IncState(value = result)
	obj_str <- Json.stringify(obj)
	<- obj_str
```

_Input config_: run indefinitely every second

```json
{
  "clock": { "start_sec": 1, "end_sec": 0, "period_sec": 1 },
  "connections": { "connect": false, "disconnect": false },
  "blockchain": { "start_block": 0, "end_block": 0 }
}
```

_Input state_: 
```json
{ "value": 0 }
```

Plan:

1. Install a spell
2. Run it for several seconds
3. Update the config to stop the execution (send an empty config)
4. Get `"counter"`
5. Wait for a spell to "work", so if it's not stopped it would increment the value.
6. Get `"value"`
7. Check that spell was executed using the counter.
8. Check that the incremented value is equal to the counter.
9. Remove the spell

### Test spell installation

_Input script_: script does nothing

_Input config_: oneshot config that runs immediately

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

### Test spell removal

Creates additional service which is used as an independant storage (we use a stopped spell for that).

_Input script_:
```aqua
service JsonStr("json"):
  parse(str: string) -> string

service JsonNum("json"):
  stringify(obj: i64) -> string
  parse(str: string) -> i64

service Json("json"):
  stringify(obj: ⊤) -> string

func inc_other_spell(fellow_spell_id: string):
    fellow_spell_id_real <- JsonStr.parse(fellow_spell_id)
    Spell fellow_spell_id_real
    result <- Spell.get_string("value")
    if result.success:
        value_num <- JsonNum.parse(result.str)
        value_new <- Math.add(value_num, 1)
        value_str <- JsonNum.stringify(value_new)
        Spell.set_string("value", value_str)

```

_Input state_:
```json
{ "value": 0 }
```

#### Remove running

Remove a spell which is subscribed to triggers at the moment.

_Input config_: connection trigger

After remove:
1. the spell is unavailable via its `spell_id`
2. no action are executed by the spell: the other service isn't affected by the
   spell anymore

#### Remove never run

_Input config_: empty config

After remove:
1. the spell is unavailable via its `spell_id`.
2. the spell stopped execution: the aux service isn't affected by the spell.

#### Remove stopped

_Input config_: any type that runs immediately

1. Stop the spell; remove.
2. the spell is unavailable via its `spell_id`.
3. the spell stopped execution: the aux service isn't affected by the spell.

### Test Remove API

*Input*: any correct input

Checks:
1. remove a spell via `srv.remove` is failing
2. `spell.remove` can't remove non-spell services (TODO: need to implement good way of obtaining a simple service first)

## Test Spell List

Note that `list` doesn't show running/stopped spells, just the installed ones.

Checks:
1. After installing the spell, its id is in the list.
2. After removing the spell, its id isn't in the list.

## Test trigger config updates

### Basic functionallity

_Input script_: simple script

Plan:
1. Check that trigger mailbox is empty and its counter is empty.
2. Set oneshot config.
3. Check that config is updated via `get_trigger_config`.
3. Check that trigger mailbox contains one timer trigger and its counter is one.

### Permissions

_Input script_: do nothing

_Input config_: empty

1. Try to update the spell with different sk and fail.

## Test trigger mailbox

_Input script_: any

_Input config_: empty

### Oneshot triggers

1. Check that the trigger mailbox is empty and the counter is zero.
2. Set oneshot config.
3. Check that spell was triggered by timer
4. Check the counter is one.

### Periodic triggers

1. Run periodic spell and stop it after a while.
2. Retrieve all entried from the trigger mailbox.
3. Check that all of them were timer triggers.
4. Check that the spell's counter isn't zero
5. Check that number of triggers is equal to the counter.

### Connections triggers

1. Set connection trigger config.
2. Trigger connect event.
3. Check the latest trigger from the trigger mailbox.
4. Update to disconnect trigger config.
2. Trigger disconnect event.
6. Check the latest trigger from the trigger mailbox.

## Test Configs

Note that we already checked oneshot configs.

_Input script_: any

### Timer config: periodic

_Input config_: periodic, 1 sec, that runs immediately

Checks:
1. Set periodic config and wait until the spell is executed several times.
2. Get two last trigger events, compare the difference between the timestamps aren't much bigger or less then the set period.

### Timer config: start_sec

_Input config_: periodic, 1 sec, which starts in the far future.

1. Check that the spell wasn't executed after setting the config.

Note: a timer config consider non empty of the `start_sec` isn't zero, so every config in this test set uses `start_sec`.
Here we want to be sure that `start_sec` isn't ignored at all.

### Timer config: periodic and end_sec

_Input config_: periodic, 1 sec, which ends in several seconds

Checks:

1. Wait until `end_sec`
2. Retrieve the last trigger from the mailbox.
3. The trigger's timestamp must be less or equal to `end_sec`
4. Check that spell is stopped.

### Bad configs

Check that all incorrect configs are rejected. Right now only ClockConfig can be
bad.

Incorrect configs:

- `end_sec < start_sec`
- `end_sec` is in the past
- `period_sec` is very big


## Test spell error API

_Input script_:
```air
(xor
    (call %init_peer_id% ("not-exist" "not-exist") [] x)
    (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1]) 
)
```

_Input config_: oneshot

1. Run spell.
2. Retrieve spell's errors via spell api `get_all_errors`
3. Check that errors exist.
4. Check that `particle_id` saved in error corresponds to real `spell_id`.
