import math, sequtils
randomize()

type
    Shot = enum
        miss, hit

proc fire() : Shot =
    let a = random(1.0)
    let b = random(1.0)
    if (a*a+b*b<1): result = hit
    else: result = miss

let max = 1000000
var shots = newSeq[Shot](max)
shots.mapIt(fire())
let hits = shots.filterIt(it == hit).len
echo (hits/max*4)