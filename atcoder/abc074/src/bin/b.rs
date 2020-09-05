#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        xx: [usize; n]
    };
    let sum = xx
        .iter()
        .fold(0, |acc, &x| acc + std::cmp::min(k - x, x) * 2);
    println!("{}", sum);
}
