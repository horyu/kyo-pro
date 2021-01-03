#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut vv: [usize; n]
    };
    vv.sort_unstable();
    let vv: Vec<f64> = vv.into_iter().map(|v| v as f64).collect();
    println!("{}", vv[1..].iter().fold(vv[0], |acc, v| (acc + v) / 2f64));
}
