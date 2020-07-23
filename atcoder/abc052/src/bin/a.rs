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
    println!("{}", std::cmp::max(a * b, c * d));
}
