#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        d: usize,
        x: usize,
        aa: [usize; n]
    };
    let mut sum = x;
    for a in aa {
        for _ in (1..=d).step_by(a) {
            sum += 1;
        }
    }
    println!("{}", sum);
}
