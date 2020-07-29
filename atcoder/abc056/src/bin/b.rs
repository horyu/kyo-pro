#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        w: isize,
        a: isize,
        b: isize
    };
    use std::cmp::max;
    println!("{}", max(max(0, a - b - w), b - a - w));
}
