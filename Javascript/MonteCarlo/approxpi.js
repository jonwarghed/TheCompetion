var max = 1000000;

function fire(){
    var a = Math.random();
    var b = Math.random();
    return {hit: a*a + b*b < 1}
}

var shots = Array.apply(0, Array(max)).map(function() { return fire()});
var hits = shots.filter(function(shot){return shot.hit;}).length;
console.log(hits/max * 4);