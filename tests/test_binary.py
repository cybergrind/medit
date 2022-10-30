import medit_rs

def test_binary():
    b = b'thisismark2'
    returned = medit_rs.test_binary(b)
    assert returned == b
    assert id(returned) != id(b)
