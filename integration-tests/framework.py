import pytest
import time
import delegator
import random
import json
import ed25519
import os
from config import get_local

def get_sk():
    return ed25519.create_keypair()[0].to_ascii(encoding="base64").decode("utf-8")

def get_relay():
    env = os.environ.get("FLUENCE_ENV")
    if env == "local":
        peers = get_local()
    else:
        if env is None:
            env = "stage"
        c = delegator.run(f"npx aqua config default_peers {env}", block=True)
        peers = c.out.strip().split("\n")

    assert len(peers) != 0, c.err
    peer = peers[random.randint(0, len(peers) - 1)]
    assert len(peer) != 0, c.err

    return peer

def get_random_peer_id():
    addr = get_relay()
    return addr.split("/")[-1]

def run_aqua(sk, func, args, relay=get_relay()):

    # "a" : arg1, "b" : arg2 .....
    data = {chr(97 + i): arg for (i, arg) in enumerate(args)}
    call = f"{func}(" + ", ".join([chr(97 + i) for i in range(0, len(args))]) + ")"
    file = "./aqua/lib.aqua"

    command = f"npx aqua run --addr {relay} -f '{call}' -i {file} --sk {sk} -d '{json.dumps(data)}'"
    print(command)
    c = delegator.run(command, block=True)
    if len(c.err) != 0:
        print(c.err)

    result = None
    if c.out != "":
        result = json.loads(c.out)
        print(result)
    return result

def get_peer_id(sk):
    return run_aqua(sk, "get_peer_id", [])

def install_spell(sk, script, config, dat):
    return run_aqua(sk, "install", [script, config, dat])

def install_spell_ok(sk, script, config, dat = "{}"):
    """
    Install a spell with given configuration and check the resulting spell_id
    """
    result = install_spell(sk, script, config, dat)
    assert result["success"]
    assert len(result["spell_id"]) != 0
    return result["spell_id"]

def remove_spell(sk, spell_id):
    return run_aqua(sk, "remove", [spell_id])

def remove_spell_ok(sk, spell_id):
    result = remove_spell(sk, spell_id)
    assert result["success"]

def update_spell(sk, spell_id, config):
    return run_aqua(sk, "update", [spell_id, config])

def update_spell_ok(sk, spell_id, config):
    result = update_spell(sk, spell_id, config)
    assert result["success"]

def create_spell(script, config, dat):
    sk = get_sk()
    spell_id = install_spell_ok(sk, script, config, json.dumps(dat))
    return spell_id, sk

def destroy_spell(sk, spell_id):
    if spell_id is not None:
        remove_spell_ok(sk, spell_id)

def with_spell(cls):
    """
    A decorator for test classes that ensures the spell is installed before the
    tests are executed and is removed after the tests finish. It does so by
    overriding the `setup_class` and `teardown_class`. If those are already
    defined, the original versions will be called: original `setup_class` will
    be called after we create the spell and original `teardown_class` will be
    called before we remove the spell. ID of the spell will be available in test classes
    via the `spell_id` variable and the secret key to operate with the spell will be available via
    the `sk` variable.
    """

    def init_param(param_name):
        param = getattr(cls, param_name, None)
        if param is None:
            raise ValueError("The test class does not define the '{param_name}' value")
        if callable(param):
            param = param()
        return param

    script = init_param("script")
    config = init_param("config")
    dat = init_param("dat")

    # update setup_class to create a sepll + calling the original one
    old_setup_class = getattr(cls, "setup_class", None)
    def setup_class(cls):
        spell_id, sk = create_spell(script, config, dat)
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
