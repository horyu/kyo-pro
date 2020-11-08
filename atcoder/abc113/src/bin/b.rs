#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        t: isize,
        a: isize,
        hh: [isize; n]
    };
    let i = (0usize..n)
        .min_by_key(|&i| (1000 * a - (1000 * t - hh[i] * 6)).abs())
        .unwrap();
    println!("{}", i + 1);
}
