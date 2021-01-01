#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [f64; n]
    };
    let rs = 1f64 / aa.iter().fold(0f64, |acc, a| acc + 1f64 / a);
    println!("{}", rs);
}
