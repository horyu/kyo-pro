#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    // (0..6).map{|i| 2**i}.reverse()
    println!(
        "{}",
        [64, 32, 16, 8, 4, 2, 1].iter().find(|&&i| i <= n).unwrap()
    );
}
