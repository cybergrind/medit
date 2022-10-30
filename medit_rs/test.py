#!/usr/bin/env python3
import os
import medit_rs

bbin = b'thisismark'
#medit_rs.get_process_handle(os.getpid())
medit_rs.test_binary(bbin)

print(f'Marked: {bbin=}')
print(f'{list(bbin)=}')
