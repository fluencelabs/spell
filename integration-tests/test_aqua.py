import pytest
from framework import *

def empty_config():
    return {
      "clock":       { "start_sec": 0, "end_sec": 0, "period_sec": 0 },
      "connections": { "connect": False, "disconnect": False},
      "blockchain":  { "start_block": 0, "end_block": 0 }
    }

def oneshot_config():
    config = empty_config()
    config["clock"]["start_sec"] = int(time.time())
    return config

def periodic_config(period_sec):
    config = empty_config()
    config["clock"]["start_sec"] = int(time.time())
    config["clock"]["period_sec"] = period_sec
    return config

def simple_script():
    return '(call %init_peer_id% ("peer" "identify") [] x)'


@with_spell
class TestSmoke:
    """
    Test some basic functionality.
    Check that it's possible to
    1. install a spell (in `with_spell)
    2. update the config to stop the execution
    3. checks that the spell is stopped
    4. remove the spell (in `with_spell`)
    """

    script = """
        (seq
            (seq
                (call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id)
                (seq
                    (call %init_peer_id% ("getDataSrv" "value") [] value1)
                    (call %init_peer_id% ("json" "parse") [value1] value)
                )
            )
            (seq
                (call %init_peer_id% ("math" "add") [value 1] result)
                (seq
                    (seq
                        (call %init_peer_id% ("json" "obj") ["value" result] result_obj)
                        (call %init_peer_id% ("json" "stringify") [result_obj] result_str)
                    )
                    (call %init_peer_id% ("callbackSrv" "response") [result_str])
                )
            )
        )
        """

    dat = {"value": 0}

    config = periodic_config(period_sec = 1)

    def test_counter(self):
        # wait a period_sec bit for a spell to work
        time.sleep(1)
        new_config = empty_config()
        update_spell_ok(self.sk, self.spell_id, new_config)

        result = run_aqua(self.sk, "get_counter", [self.spell_id])
        assert result["success"]
        counter = result['num']

        # wait a period_sec
        # If the spell wasn't updated and is still executed, then the `value` value will be incremented
        time.sleep(1)

        result = run_aqua(self.sk, "get_string", [self.spell_id, "value"])
        assert result["success"]
        value = int(result['str'])

        assert counter == value, "values should be equal since 'value' incremented each time the spell is called"
        assert counter != 0, "the spell must be executed at least once at this point"

@with_spell
class TestInstall:
    """
    Check:
    1. `install` is called in `setup_class` that is defined by the `with_spell` decorator.
        Also, we check there that the call is successful and that the spell_is is defined.
        We also check spell_id here to make this process clear and to not depend on the `with_spell` implemention.
    2. `get_config` returns the same config we use on installtion
    3. `get_script` returns the same script we use on installtion
    4. `get_conter` returns a counter and this counter is one, since the config is oneshot.
    """

    script = simple_script()
    dat = {}
    config = oneshot_config()

    def test_spell_id(self):
        assert self.spell_id is not None
        assert len(self.spell_id) != 0

    def test_get_config(self):
        cfg_result = run_aqua(self.sk, "get_config", [self.spell_id])
        assert cfg_result["success"]
        assert cfg_result["config"] == self.config, "spell's config should be equal the one that was set during installtion"

    def test_get_script(self):
        script_result = run_aqua(self.sk, "get_script", [self.spell_id])
        assert script_result["success"]
        assert script_result["source_code"] == self.script, "spell's script should be equal the one that was set during installtion"

    def test_get_count(self):
        counter_result = run_aqua(self.sk, "get_counter", [self.spell_id])
        assert counter_result["success"]
        assert counter_result['num'] == 1, "the spell should be run exactly once at this point"

class TestRemove:
    """
    Check that the spell is unavailable after removal
    """
    def test_spell_availabilty(self):
        script = simple_script()
        dat = {}
        config = oneshot_config()

        spell_id, sk = create_spell(script, config, dat)
        destroy_spell(sk, spell_id)

		# Not2 that this call may lead to errors on the node because we try to
		# resolve spell id there to check availabilty.
        result = run_aqua(sk, "is_spell_absent", [spell_id])
        assert result, "the spell should be unavailable"
