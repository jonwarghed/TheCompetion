extern crate rand;
use rand::distributions::{IndependentSample, Range};
use std::old_io::stdin;

fn main() {
    let total = 1_000_000;
    let hits = Infinity
    .take(total)
    .filter(|shot| shot.hit)
    .collect::<Vec<Shot>>().len();
    println!("{}", ((hits as f64) * 4.0)/(total as f64));
}
//Empty struct not allowed, but how can i otherwise make the iterator?
struct Infinity;
struct Shot {hit: bool}


impl Iterator for Infinity {
    type Item = Shot;
    fn next(&mut self) -> Option<Shot> {
        let mut rng = rand::thread_rng();
        let between = Range::new(0f64, 1.);
        let a = between.ind_sample(&mut rng);
        let b = between.ind_sample(&mut rng);
        Some(Shot{hit: a*a + b*b <= 1.})
    }
}

#[test]
fn should_be_able_to_fire(){
    let result = Infinity.take(1);
}