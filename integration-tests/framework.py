import pytest
import time
import delegator
import random
import json
import ed25519
import os
import tempfile
from config import get_local

def get_sk():
    return ed25519.create_keypair()[0].to_ascii(encoding="base64").decode("utf-8")

def get_relays():
    env = os.environ.get("FLUENCE_ENV")
    if env == "local":
        peers = get_local()
    else:
        if env is None:
            env = "stage"
        c = delegator.run(f"npx aqua config default_peers {env}", block=True)
        peers = c.out.strip().split("\n")

    assert len(peers) != 0, c.err
    return peers

def get_relay():
    peers = get_relays()
    peer = peers[random.randint(0, len(peers) - 1)]
    assert len(peer) != 0
    return peer

def from_aqua(aqua, func_name):
    if len(aqua) == 0 or len(func_name) == 0:
        raise ValueError("from_aqua: Empty aqua script or file name")

    with tempfile.TemporaryDirectory() as dir_name:
        file_prefix = 'spell'
        aqua_file = os.path.join(dir_name, file_prefix + '.aqua')
        try:
            with open(aqua_file, 'w') as file:
                file.write(aqua)
        except e:
            raise Exception(f"Unable to write aqua script to file by path {aqua_file}: {e}")

        target_dir = dir_name
        command_compile = f"npx aqua -i {aqua_file} -o {target_dir} --air --no-relay"
        print(command_compile)

        c = delegator.run(command_compile, block=True)
        if len(c.err) != 0:
            print(c.err)

        if c.return_code != 0:
            raise Exception(f"Unable to compile the aqua spell with name {func_name} in {file_name}")

        air_filename = file_prefix + '.' + func_name + '.air'
        air_path = os.path.join(dir_name, air_filename)
        try:
            with open(air_path) as f:
                air_script = f.read()
        except e:
            raise Exception(f"Unable to read compiled air script by path {air_path}: {e}")
        return air_script

# TODO: learn how to chose the relay based on the test worker id.
def run_aqua(sk, func, args, relay=get_relay()):

    # "a" : arg1, "b" : arg2 .....
    data = {chr(97 + i): arg for (i, arg) in enumerate(args)}
    call = f"{func}(" + ", ".join([chr(97 + i) for i in range(0, len(args))]) + ")"
    file = "./aqua/lib.aqua"

    command = f"npx aqua run --addr {relay} -f '{call}' -i {file} --sk {sk} -d '{json.dumps(data)}' --timeout 100000"
    print(command)
    c = delegator.run(command, block=True)
    if len(c.err) != 0:
        print(c.err)

    if c.return_code != 0:
        raise RuntimeError(f"can't run `{func}` in aqua due to external error. See logs to know more.")

    result = None
    if c.out != "":
        print(c.out)
        result = json.loads(c.out)
        print("Result:", result)
    return result

def get_peer_id(sk):
    return run_aqua(sk, "get_peer_id", [])

def trigger_connect():
    run_aqua(get_sk(), "noop", [])

def install_spell(sk, script, config, dat):
    return run_aqua(sk, "install", [script, config, dat])

def install_spell_ok(sk, script, config, dat = {}):
    """
    Install a spell with given configuration and check the resulting spell_id
    """
    result = install_spell(sk, script, config, dat)
    assert result["success"], "can't install spell"
    assert len(result["spell_id"]) != 0, "spell_id must not be empty"
    return result["spell_id"]

def remove_spell(sk, spell_id):
    return run_aqua(sk, "remove", [spell_id])

def remove_spell_ok(sk, spell_id):
    result = remove_spell(sk, spell_id)
    assert result["success"], f"can't remove spell {spell_id}"

def update_spell(sk, spell_id, config):
    return run_aqua(sk, "update", [spell_id, config])

def update_spell_ok(sk, spell_id, config):
    result = update_spell(sk, spell_id, config)
    assert result["success"], f"can't update the spell {spell_id}"

def get_trigger_event_ok(sk, spell_id):
    [trigger, error] = run_aqua(sk, "get_trigger_event", [spell_id])
    assert len(error) == 0, f"get_trigger_event: got error while retrieving triggers for spell {spell_id}: {error}"
    return trigger

def get_counter_ok(sk, spell_id):
    counter_result = run_aqua(sk, "get_counter", [spell_id])
    assert counter_result["success"], "get_counter failed"
    return counter_result['num']

def create_spell(script, config, dat):
    sk = get_sk()
    print("dat is", dat)
    spell_id = install_spell_ok(sk, script, config, dat)
    return spell_id, sk

def destroy_spell(sk, spell_id):
    if spell_id is not None:
        remove_spell_ok(sk, spell_id)

def with_spell(cls):
    """
    A decorator for test classes that ensures the spell is installed before the
    tests are executed and is removed after the tests finish.

    It does so by overriding the `setup_class` and `teardown_class`. If those
    are already defined, the original versions will be called: original `setup_class`
    will be called after we create the spell and original `teardown_class` will be
    called before we remove the spell.

    ID of the spell will be available in test classes via the `spell_id` variable and
    the secret key to operate with the spell will be available via the `sk` variable.

    The underlying class MUST define `air_script`, `config` and `dat` variables with
    corresponding data for installation.
    """

    def init_param(param_name):
        param = getattr(cls, param_name, None)
        if param is None:
            raise ValueError(f"The test class does not define the '{param_name}' value")
        if callable(param):
            param = param()
        return param

    air_script = init_param("air_script")
    config = init_param("config")
    dat = init_param("dat")

    # update setup_class to create a sepll + calling the original one
    old_setup_class = getattr(cls, "setup_class", None)
    def setup_class(cls):
        spell_id, sk = create_spell(air_script, config, dat)
        cls.spell_id = spell_id
        cls.sk = sk

        if old_setup_class is not None:
            old_setup_class()

    cls.setup_class = setup_class

    # update teardown_class to remove a sepll + calling the original one
    old_teardown_class = getattr(cls, "teardown_class", None)
    def teardown_class(cls):
        if old_teardown_class is not None:
            old_teardown_class()
        destroy_spell(cls.sk, cls.spell_id)

    cls.teardown_class = teardown_class

    return cls


def with_spell_each(cls):
    """
    Decorator like `with_spell`, but instead of `setup_class/teardown_class` creates
    `setup_method/teardown_method` which are called for every test instead of for all class
    """

    def init_param(param_name):
        param = getattr(cls, param_name, None)
        if param is None:
            raise ValueError(f"The test class does not define the '{param_name}' value")
        if callable(param):
            param = param()
        return param

    air_script = init_param("air_script")
    config = init_param("config")
    dat = init_param("dat")

    # update setup_class to create a sepll + calling the original one
    old_setup_method = getattr(cls, "setup_method", None)
    def setup_method(cls):
        spell_id, sk = create_spell(air_script, config, dat)
        cls.spell_id = spell_id
        cls.sk = sk

        if old_setup_method is not None:
            old_setup_method()

    cls.setup_method = setup_method

    # update teardown_method to remove a sepll + calling the original one
    old_teardown_method = getattr(cls, "teardown_method", None)
    def teardown_method(cls):
        if old_teardown_method is not None:
            old_teardown_method()
        destroy_spell(cls.sk, cls.spell_id)
        cls.spell_id = None
        cls.sk = None

    cls.teardown_method = teardown_method

    return cls
