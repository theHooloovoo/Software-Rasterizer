#!/usr/bin/env python3

import random
import sys

if len(sys.argv) != 4:
    print("This script requires three positive integers as inputs:")
    print("  - Maximum x value allowed")
    print("  - Maximum y value allowed")
    print("  - Number of triangles to generate")

n = int(sys.argv[3])
x = float(sys.argv[1])
y = float(sys.argv[2])

for _ in range(0, n):
    print(random.uniform(0, x), random.uniform(0,y),
          random.uniform(0, x), random.uniform(0,y),
          random.uniform(0, x), random.uniform(0,y) )
