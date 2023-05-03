import delegator
import warnings


def test_aqua_version():
    c = delegator.run(f"npx fluence --version", block=True)
    print(f"Fluence cli version: {c.out}")
    warnings.warn(f"Fluence cli version: {c.out}")
    assert True
