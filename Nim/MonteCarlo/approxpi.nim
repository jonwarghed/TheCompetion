import math
randomize()

type
    hit = bool

proc fire() : hit =
    let a = random(1.0)
    let b = random(1.0)
    result = a*a+b*b<1

let max = 100000
let list = (0..max).map(fire)