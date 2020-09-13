#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    };
    use std::cmp::max;
    println!("{}", max(max(a * c, a * d), max(b * c, b * d)));
}
