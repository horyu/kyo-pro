#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: usize,
        t: usize
    };
    println!("{}", t - s + 1);
}
