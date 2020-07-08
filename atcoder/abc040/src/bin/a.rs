#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        x: usize
    };
    println!("{}", if x > n / 2 { n - x } else { x - 1 });
}
