import subprocess
import pytest
from importlib import reload, import_module


@pytest.fixture(scope='session', autouse=True)
def compile():
    """
    if you've changed binary you need to recompile it
    """
    #subprocess.run(['make', 'compile-dev'], check=True)

