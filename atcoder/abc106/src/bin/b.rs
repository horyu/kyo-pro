#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let rs = (1..=n)
        .step_by(2)
        .filter(|&i| (1..=i).step_by(2).filter(|&j| i % j == 0).count() == 8)
        .count();
    println!("{}", rs);
}
