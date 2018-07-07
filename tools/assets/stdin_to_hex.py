#!/usr/bin/env python3
# a simple script to just visualize a sheet or sumthin.
import sys

break_line = 48 # default for sprite sheets in bity-8
h_space = 3       # default for fonts
v_space = 6       # default for fonts

if len(sys.argv) > 1:
    break_line = int(sys.argv[1])

data = sys.stdin.buffer.read()
for x in sys.argv:
    print(x)

for i, x in enumerate(data):
    if i % break_line == 0:
        print()

    if i % (v_space*break_line) == 0:
        print()

    if i % h_space == 0:
        sys.stdout.write(' ')

    val = '{:02x}'.format(x).replace('0', ' ')
    sys.stdout.write(val)

print()
