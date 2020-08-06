#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: u32,
        k: usize
    };
    println!("{}", k * (k - 1).pow(n - 1));
}
