#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        llrr: [(usize, usize); n]
    };
    let sum = llrr.iter().fold(0, |acc, &(l, r)| acc + r - l + 1);
    println!("{}", sum);
}
