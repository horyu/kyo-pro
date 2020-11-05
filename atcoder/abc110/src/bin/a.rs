#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    use std::cmp::max;
    println!(
        "{}",
        max(10 * a + b + c, max(10 * b + c + a, 10 * c + a + b))
    );
}
