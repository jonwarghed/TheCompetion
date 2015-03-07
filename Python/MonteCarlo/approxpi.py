##!/bin/env python
#This code is both python 2.7 and 3.x compatible
from collections import namedtuple
import random

Shot = namedtuple("Shot", "hit")

def fire():
    a = random.random()
    b = random.random()
    return Shot(hit=a*a+b*b<1.0)

max = 1000000
shots = [fire() for target in range(max)]
hits = len([shot.hit for shot in shots if shot.hit])
print(float(hits)/max*4)