#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aabb: [(usize, usize); n]
    };
    let rs = aabb
        .iter()
        .fold(0, |acc, (a, b)| acc + (b * b + b) / 2 - (a * a - a) / 2);
    println!("{}", rs);
}
