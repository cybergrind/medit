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
    scanner = medit_rs.gen_scanner()
