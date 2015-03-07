

function fire(){
    var a = Math.random();
    var b = Math.random();
    return {hit: a*a + b*b < 1}
}

var shots = [];
var max = 1000000;
for (var i = 0; i <= max; i++) {
    shots.push(fire());
}
var hits = shots.filter(function(shot){return shot.hit;}).length;
console.log(hits/max * 4);