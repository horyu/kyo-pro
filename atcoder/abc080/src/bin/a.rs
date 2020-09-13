#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    };
    println!("{}", std::cmp::min(n * a, b));
}
