class Shot
    constructor: (@hit) ->

fire = () ->
    a = Math.random()
    b = Math.random()
    new Shot(a*a + b*b < 1)

max = 1000000;
shots = (fire() for attempt in [1..max])
hits = (shot for shot in shots when shot.hit).length
console.log hits/max * 4