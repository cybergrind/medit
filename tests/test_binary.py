import os

import medit_rs


def test_binary():
    b = b'thisismark2'
    returned = medit_rs.test_binary(b)
    assert returned == b
    assert id(returned) != id(b)


def test_mem_maps():
    pid = os.getpid()
    maps = medit_rs.get_mem_maps(pid)
    assert maps != {}


def test_scanner():
    b = b'iabinary'
    scanner = medit_rs.PyScanner(os.getpid())
    assert scanner
    # 32:32 + len(b)
    found = bytes(scanner.read(id(b), 40))[32:40]
    assert found == b
