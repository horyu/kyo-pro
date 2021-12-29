#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut rr: [usize; n]
    };
    rr.sort_unstable();
    rr.reverse();
    let mut sum = 0.0;
    for (i, &r) in rr.iter().enumerate() {
        let r = r as f64;
        if i % 2 == 0 {
            sum += r * r;
        } else {
            sum -= r * r;
        }
    }
    println!("{}", std::f64::consts::PI * sum);
}
