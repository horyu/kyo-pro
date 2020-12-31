#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize
    };
    use std::cmp::max;
    println!("{}", max(a + b, max(a - b, a * b)));
}
