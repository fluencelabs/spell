import delegator
import warnings


def test_aqua_version():
    c = delegator.run(f"npx aqua --version", block=True)
    print(f"Aqua version: {c.out}")
    warnings.warn(f"Aqua version: {c.out}")
    assert True
