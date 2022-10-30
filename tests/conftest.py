import subprocess
import pytest


@pytest.fixture(scope='session', autouse=True)
def compile():
    subprocess.run(['make', 'compile-dev'], check=True)
