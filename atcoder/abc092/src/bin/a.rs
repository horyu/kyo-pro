#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    use std::cmp::min;
    println!("{}", min(a, b) + min(c, d));
}
