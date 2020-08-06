#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        d: usize
    };
    println!("{}", std::cmp::max((a + 1) * d, a * (d + 1)));
}
