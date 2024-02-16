import pytest
from framework import *
import time

def empty_config():
    return {
        "clock": {"start_sec": 0, "end_sec": 0, "period_sec": 0},
        "connections": {"connect": False, "disconnect": False},
        "blockchain": {"start_block": 0, "end_block": 0}
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

def connect_config():
    config = empty_config()
    config["connections"]["connect"] = True
    return config

def simple_script():
    return '(call %init_peer_id% ("peer" "identify") [] x)'

def store_triggers_script():
    return '''
    (seq
        (call %init_peer_id% ("getDataSrv" "hw_trigger") [] trigger)
        (seq
            (call %init_peer_id% ("json" "stringify") [trigger] trigger_str)
            (call %init_peer_id% ("spell" "list_push_string") ["triggers" trigger_str])
        )
    )'''


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

    # Air: air/test_spells.inc_value.air
    # Aqua: aqua/test_spells.aqua, function: inc_value
    air_script = open("./air/test_spells.inc_value.air").read()

    dat = {"value": 0}

    config = periodic_config(period_sec=1)

    def test_smoke_counter(self):
        # wait a period_sec bit for a spell to work
        time.sleep(1)
        new_config = empty_config()
        update_spell_ok(self.key_pair_name, self.spell_id, new_config)

        counter = get_counter_ok(self.key_pair_name, self.spell_id)

        # wait a period_sec
        # If the spell wasn't updated and is still executed, then the `value` value will be incremented
        time.sleep(1)

        result = run_aqua(self.key_pair_name, "get_string", [self.spell_id, "value"])
        assert result["success"]
        value = int(result['value'])

        assert counter == value, "values should be equal since 'value' incremented each time the spell is called"
        assert counter != 0, "the spell must be executed at least once at this point"

@with_spell
class TestInstall:
    """
    Check:
    1. `install` is called in `setup_class` that is defined by the `with_spell` decorator.
        Also, we check there that the call is successful and that the spell_id is defined.
        We also check spell_id here to make this process clear and to not depend on the `with_spell` implemention.
    2. `get_config` returns the same config we use on installtion
    3. `get_script` returns the same script we use on installtion
    4. `get_conter` returns a counter and this counter is one, since the config is oneshot.
    """

    air_script = simple_script()
    dat = {}
    config = oneshot_config()

    def test_install_spell_id(self):
        assert self.spell_id is not None
        assert len(self.spell_id) != 0

    def test_install_get_config(self):
        cfg_result = run_aqua(self.key_pair_name, "get_config", [self.spell_id])
        assert cfg_result["success"]
        assert cfg_result["config"] == self.config, "spell's config should be equal the one that was set during installtion"

    def test_install_get_script(self):
        script_result = run_aqua(self.key_pair_name, "get_script", [self.spell_id])
        assert script_result["success"]
        assert script_result["value"] == self.air_script, "spell's script should be equal the one that was set during installtion"

    def test_install_get_count(self):
        counter_result = run_aqua(self.key_pair_name, "get_counter", [self.spell_id])
        assert counter_result["success"]
        assert counter_result['value'] == 1, "the spell should be run exactly once at this point"
        counter = get_counter_ok(self.key_pair_name, self.spell_id)

    # TODO: what is it and how is it working?
    def _test_install_location(self):
        pass

@with_spell
class TestRemoveApi:
    """
    Check the node API behaivoir:
        1. srv.remove can't remove a spell
        2. spell.remove can't remove a service (TODO)
    """

    air_script = simple_script()
    dat = {}
    config = empty_config()

    def test_remove_spell(self):
        result = run_aqua(self.key_pair_name, "remove_service", [self.spell_id])
        assert not result["success"]

@with_spell
class TestRemoveWithAux:
    """
    Here we create two spells:
    1) the first one is used for storing info from the spell we are testing.
    2) the second one is the test subject.

    The second spell will be sending messages the first one on triggering:
    it will increase the "value" argument of the first spell.
    """

    VALUE_KEY = "w_value"

    # the script of the first, supporing spell can be anything
    air_script = simple_script()
    dat = {VALUE_KEY: 0}
    config = empty_config()

    worker_spell_id = None
    worker_key_pair_name = None

    # setup here the second spell that will be sending things to the first.
    def setup_method(self):
        # Aqua: aqua/test_spells.aqua, func: inc_other_spell
        # Air: air/test_spells.inc_other_spell.air
        script = open("./air/test_spells.inc_other_spell.air").read()

        config = empty_config()
        # pass the storage spell id to the worker spell
        dat = {"fellow_spell_id": self.spell_id}

        spell_id = create_spell(self.key_pair_name, script, config, dat)
        self.worker_spell_id = spell_id

    def run_scenario(self):
        # remove spell stopping it
        destroy_spell(self.key_pair_name, self.worker_spell_id)

        # check that spell isn't available by its spell id
        result = run_aqua(self.key_pair_name, "is_spell_absent", [self.worker_spell_id])
        assert result, "the spell should be unavailable"

        # get value from the aux spell
        result = run_aqua(self.key_pair_name, "get_string", [self.spell_id, self.VALUE_KEY])
        assert result["success"]
        assert not result["absent"]
        value = result["value"]

        trigger_connect()

        result = run_aqua(self.key_pair_name, "get_string", [self.spell_id, self.VALUE_KEY])
        assert result["success"]
        assert not result["absent"]
        value2 = result["value"]

        assert value == value2, "the worker spell must be stopped"


    def test_remove_never_run(self):
        # the spell initially is created with empty config so it's never run
        self.run_scenario()

    def test_remove_stopped(self):
        # run spell
        update_spell_ok(self.key_pair_name, self.worker_spell_id, connect_config())
        # trigger spell
        trigger_connect()
        # stop spell
        update_spell_ok(self.key_pair_name, self.worker_spell_id, empty_config())

        self.run_scenario()

    def test_remove_running(self):
        # run the spell
        update_spell_ok(self.key_pair_name, self.worker_spell_id, connect_config())

        result = run_aqua(self.key_pair_name, "get_string", [self.spell_id, self.VALUE_KEY])
        assert result["success"]
        assert not result["absent"]
        value1 = result["value"]

        # trigger spell
        trigger_connect()

        result = run_aqua(self.key_pair_name, "get_string", [self.spell_id, self.VALUE_KEY])
        assert result["success"]
        assert not result["absent"]
        value2 = result["value"]

        assert value1 != value2, "spell must be run once"


        self.run_scenario()

class TestList:
    def test_list(self):
        key_pair_name = make_key()
        spell_id = create_spell(key_pair_name, simple_script(), empty_config(), {})
        spells_after_install = run_aqua(key_pair_name, "list_spells", [])
        destroy_spell(key_pair_name, spell_id)
        spells_after_remove = run_aqua(key_pair_name, "list_spells", [])

        assert spell_id in spells_after_install, "spell_id must be in the list of spells after spell installation"
        assert spell_id not in spells_after_remove, "spell_id must NOT be in the list of spells after spell removal"

@with_spell
class TestUpdate:
    air_script = simple_script()
    config = empty_config()
    dat = {}

    def test_update_forbid(self):
        other_key_pair_name = make_key()
        result = update_spell(other_key_pair_name, self.spell_id, empty_config())
        assert not result["success"], "spell is allowed to be updated only by owner"

    def test_update_config(self):
        config_expected = oneshot_config()

        update_spell_ok(self.key_pair_name, self.spell_id, config_expected)

        config_result = run_aqua(self.key_pair_name, "get_config", [self.spell_id])
        assert config_result["success"], "can't retrive spell config"
        assert config_expected == config_result["config"], "spell's config must change after update"

        trigger = get_trigger_event_ok(self.key_pair_name, self.spell_id)
        assert len(trigger["timer"]) == 1, "spell must be subscribed to timer trigger which must happen at this time"

@with_spell_each
class TestTriggerMailbox:
    air_script = store_triggers_script()
    config = empty_config()
    dat = {}

    def test_triggers_oneshot(self):
        trigger = get_trigger_event_ok(self.key_pair_name, self.spell_id)

        assert trigger is None, "no triggers must be in the spell's trigger mailbox on empty config"

        counter = get_counter_ok(self.key_pair_name, self.spell_id)
        assert counter == 0, "the spell must NOT be run"

        update_spell_ok(self.key_pair_name, self.spell_id, oneshot_config())

        trigger = get_trigger_event_ok(self.key_pair_name, self.spell_id)
        assert trigger is not None, "trigger should be retrived"

        assert trigger["peer"] is None, "peer trigger must NOT happen"
        assert len(trigger["timer"]) == 1, "timer trigger must happen"

        counter = get_counter_ok(self.key_pair_name, self.spell_id)
        assert counter == 1, "the spell must be run"

    def test_triggers_periodic(self):
        update_spell_ok(self.key_pair_name, self.spell_id, periodic_config(1))
        time.sleep(1)
        update_spell_ok(self.key_pair_name, self.spell_id, empty_config())

        trigger = get_trigger_event_ok(self.key_pair_name, self.spell_id)
        assert len(trigger) != 0, "trigger should be retrived"
        assert trigger["timer"] is not None, "timer trigger must happen"
        assert trigger["peer"] is None, "peer trigger must NOT happen"


    def test_triggers_connections(self):
        config = empty_config()
        config["connections"]["connect"] = True
        update_spell_ok(self.key_pair_name, self.spell_id, config)

        trigger_connect()

        trigger = get_trigger_event_ok(self.key_pair_name, self.spell_id)
        assert len(trigger) != 0, "trigger should be retrived"

        assert trigger["peer"] is not None, "peer trigger must happen"
        assert trigger["timer"] is None, "timer trigger must NOT happen"

        assert trigger["peer"]["connected"], "spell must be trigger by connected event"

        config = empty_config()
        config["connections"]["disconnect"] = True
        update_spell_ok(self.key_pair_name, self.spell_id, config)

        trigger_connect()

        trigger = get_trigger_event_ok(self.key_pair_name, self.spell_id)
        assert trigger is not None, "trigger should be retrived"

        assert trigger["peer"] is not None, "peer trigger must happen"
        assert trigger["timer"] is None, "timer trigger must NOT happen"

        assert not trigger["peer"]["connected"], "spell must be trigger by connected event"


@with_spell_each
class TestConfig:
    air_script = store_triggers_script()
    config = empty_config()
    dat = {}

    # actually check periods between triggers
    def test_config_periodic(self):
        period_expected = 1
        update_spell_ok(self.key_pair_name, self.spell_id, periodic_config(period_expected))
        timestamp1 = int(time.time())
        time.sleep(period_expected)
        trigger = get_trigger_event_ok(self.key_pair_name, self.spell_id)
        assert trigger is not None, "trigger should be retrived"
        timestamp2 = trigger['timer']['timestamp']

        period_result = abs(timestamp1 - timest)
        assert period_result >= period_expected, "real period is less then configured: real: {period_result}, expected: {period_expected} "
        assert period_result <= period_expected + 1, "real period is much larger then configures: real: {period_result}, expected: {period_expected} "

    def test_config_bad(self):
        def check(config):
            result = update_spell(self.key_pair_name, self.spell_id, config)
            assert not result["success"], "bad config must not be set"

        bad = empty_config()
        bad['clock']['start_sec'] = int(time.time() + 10000)
        bad['clock']['end_sec'] = bad['clock']['start_sec'] - 1
        check(bad)

        bad = empty_config()
        bad['clock']['start_sec'] = int(time.time() - 100000)
        bad['clock']['end_sec'] = bad['clock']['start_sec'] + 1
        check(bad)

        bad = periodic_config(period_sec=100 * 365 * 24 * 60 * 60 + 100)
        check(bad)

    def test_config_start_sec(self):
        # late start
        config = oneshot_config()
        config['clock']['start_sec'] = int(time.time()) + 60 * 10

        update_spell_ok(self.key_pair_name, self.spell_id, config)

        counter = get_counter_ok(self.key_pair_name, self.spell_id)
        assert counter == 0, "spell must NOT be run"

	# Right now the spell with `end_sec` checks `now < end_sec`, not `now + period < end_sec`, so
	# the test failing. Need to fix the node.
    def _test_config_end_sec(self):
        wait_sec = 8
        config = periodic_config(3)
        config['clock']['end_sec'] = int(time.time()) + wait_sec
        update_spell_ok(self.key_pair_name, self.spell_id, config)
        time.sleep(wait_sec)

        trigger = get_trigger_event_ok(self.key_pair_name, self.spell_id)
        assert len(trigger) == 1, "the spell must be triggered at this point"
        assert trigger[0]['timer'][0]['timestamp'] <= config['clock']['end_sec'], "the spell was triggered after `end_sec`"
        # TODO: check that the spell is stopped

@with_spell
class TestSpellError:
    air_script = """
    (xor
        (call %init_peer_id% ("not-exist" "not-exist") [] x)
        (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
    )
    """
    config = oneshot_config()
    dat = {}

    def test_error(self):
        result = run_aqua(self.key_pair_name, "get_spell_errors", [self.spell_id])
        print(result)
        assert result["success"], "get_spell_errors failed"
        errors = result["particle_errors"]

        assert len(errors) == 1, "spell was executed once and must produce only one error"

        assert self.spell_id in errors[0]["particle_id"], "error must belong to spell_id"

@with_spell
class TestSpellStatus:
    """
    Check installation-spell status API

    Since we can't change the status directly, we send message to the spell's mailbox so the spell
    sets the required status itself.
    """
    air_script = open("./air/test_spells.status_spell.air").read()

    dat = {}
    config = connect_config()
    alias = "worker-spell"

    def test_status(self):
        trigger_connect()

        status = run_aqua(self.key_pair_name, "get_worker_spell_status", [])
        assert status["state"] == "NOT_STARTED"
        assert status["message"] == "Installation has not started yet"

        statuses = run_aqua(self.key_pair_name, "get_worker_spell_statuses_from", [0])
        assert len(statuses) == 1
        assert statuses[0]["state"] == "NOT_STARTED"
        assert statuses[0]["message"] == "Installation has not started yet"

        run_aqua(self.key_pair_name, "send_worker_spell_status", ["INSTALLATION_IN_PROGRESS", "installation in progress"])
        trigger_connect()

        first_status = run_aqua(self.key_pair_name, "get_worker_spell_status", [])
        assert first_status["state"] == "INSTALLATION_IN_PROGRESS"
        assert first_status["message"] == "installation in progress"
        assert first_status["timestamp"] > 0

        statuses = run_aqua(self.key_pair_name, "get_worker_spell_statuses_from", [0])
        assert len(statuses) == 1
        assert statuses[0] == first_status

        time.sleep(2)
        run_aqua(self.key_pair_name, "send_worker_spell_status", ["INSTALLATION_SUCCESSFUL", "installation finished"])
        trigger_connect()

        last_status = run_aqua(self.key_pair_name, "get_worker_spell_status", [])
        assert last_status["state"] == "INSTALLATION_SUCCESSFUL"
        assert last_status["message"] == "installation finished"
        assert last_status["timestamp"] > 0

        statuses = run_aqua(self.key_pair_name, "get_worker_spell_statuses_from", [0])
        assert len(statuses) == 2
        assert statuses[0] == first_status
        assert statuses[1] == last_status

        statuses = run_aqua(self.key_pair_name, "get_worker_spell_statuses_from", [last_status["timestamp"]])
        assert len(statuses) == 1
        assert statuses[0] == last_status

class TestSpellKvPermissions:
    """
    1. Create two spells on one worker (should be able to change w_ and hw_ keys and can't change any other keys on each other)
    2. Create a spell on another worker (shouldn't be able to change anything on the original spells)
    3. We can't create a root spells, so we can't check `h_` key, but we can check that only a spell itself can write to the key


    A spell on the first worker is:
    - trying to increment w_test_counter and hw_test_counter value on the fellow worker (and succeed)
    - trying to increment test_counter and h_test_counter on the fellow worker (and fail)

    A spell on another worker is:
    - trying to modify test_counter_2, w_test_counter_2, hw_test_counter_2 on the spell on worker 1 and fails 
    """

    VALUE = "test_counter"
    W_VALUE = "w_test_counter"
    H_VALUE = "h_test_counter"
    HW_VALUE = "hw_test_counter"

    VALUE_2 = "test_counter_2"
    W_VALUE_2 = "w_test_counter_2"
    H_VALUE_2 = "h_test_counter_2"
    HW_VALUE_2 = "hw_test_counter_2"

    @classmethod
    def setup_worker_1(self):
        dat = {
            self.VALUE: 0,
            self.W_VALUE: 0,
            self.H_VALUE: 0,
            self.HW_VALUE: 0,

            self.VALUE_2: 0,
            self.W_VALUE_2: 0,
            self.H_VALUE_2: 0,
            self.HW_VALUE_2: 0
        }
        script = open("air/test_spells.actor_spell.air").read()

        worker_1_key = make_key()

        # Important to create subject first so the actor can access it
        spell_id2 = create_spell(worker_1_key, simple_script(), empty_config(), dat, "subject")
        spell_id1 = create_spell(worker_1_key, script, oneshot_config(), dat, "actor")

        worker_id = run_aqua(worker_1_key, "get_worker_id_on_host", [])

        self.worker_1_key = worker_1_key
        self.worker_1_id = worker_id.strip()

    @classmethod
    def setup_worker_2(self):
        script = open("air/test_spells.stranger_spell.air").read()
        dat = {"target_worker_id": self.worker_1_id}
        worker_2_key = make_key()
        spell_id_2 = create_spell(worker_2_key, script, oneshot_config(), dat, "stranger")
        self.worker_2_key = worker_2_key

    def setup_class(self):
        # important to create worker_1 first, since worker_2 uses it
        self.setup_worker_1()
        self.setup_worker_2()

    def teardown_class(self):
        run_aqua(self.worker_1_key, "remove_worker", [])
        run_aqua(self.worker_2_key, "remove_worker", [])

    def test_kv_permissions(self):
        # at this point all the spells should already end

        read_reports, write_reports = run_aqua(self.worker_1_key, "get_reports", ["actor"])
        assert all(map(lambda x: x["result"], read_reports)), "actor should be able to read every key"
        # Actor should be able to write only w_ and hw_
        ok_writes = list(map(lambda x: x["key"], filter(lambda x: x["result"], write_reports)))
        assert all(map(lambda x: x.startswith("w_") or x.startswith("hw_"), ok_writes)), "actor should be able to change only w_ and hw_ keys"

        failed_writes = list(map(lambda x: x["key"], filter(lambda x: not x["result"], write_reports)))
        assert all(map(lambda x: not (x.startswith("w_") or x.startswith("hw_")), failed_writes)), "actor should NOT be able to change any other keys"


        # Stranger should be able to read everything and write nothing
        read_reports, write_reports = run_aqua(self.worker_2_key, "get_reports", ["stranger"])
        assert all(map(lambda x: x["result"], read_reports)), "stranger should be able to read every key"
        assert all(map(lambda x: not x["result"], write_reports)), "stranger should NOT be able to write any key"

        result = run_aqua(self.worker_1_key, "get_target_values", ["subject"])
        # Subject's worker key should be changed
        assert result[self.W_VALUE] != 0
        assert result[self.HW_VALUE] != 0
        # Subject's private and host keys must not be changed
        assert result[self.VALUE] == 0
        assert result[self.H_VALUE] == 0
        # Subject's _2 keys must not be changed
        assert result[self.VALUE_2] == 0
        assert result[self.W_VALUE_2] == 0
        assert result[self.H_VALUE_2] == 0
        assert result[self.HW_VALUE_2] == 0

        print(result)
