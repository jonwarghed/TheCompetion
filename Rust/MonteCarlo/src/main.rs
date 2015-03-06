extern crate rand;
use rand::distributions::{IndependentSample, Range};
/*
fn main() {
   let between = Range::new(0f64, 1.);
   let mut rng = rand::thread_rng();

   let total = 1_000_000;
   let mut in_circle = 0;

   for _ in 0..total {
       let a = between.ind_sample(&mut rng);
       let b = between.ind_sample(&mut rng);
       if a*a + b*b <= 1. {
           in_circle += 1;
       }
   }
   // prints something close to 3.14159...
   println!("{}", 4. * (in_circle as f64) / (total as f64));
}
*/
fn main() {
   let total = 1_000_000;
   let shots = (0..total).map(fire).collect();
}

fn fire<T>(x:T) -> FireEvent {
    let mut rng = rand::thread_rng();
    let between = Range::new(0f64, 1.);
    let a = between.ind_sample(&mut rng);
    let b = between.ind_sample(&mut rng);
    FireEvent {hit : a*a + b*b <= 1.}
}

struct FireEvent { hit: bool, }

#[test]
fn should_be_able_to_fire(){
    let result = fire();
}