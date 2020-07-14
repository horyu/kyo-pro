#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp;

fn main() {
    input! {
        n: isize,
        k: isize,
        x: isize,
        y: isize
    };
    println!("{}", x * cmp::min(n, k) + y * cmp::max(n - k, 0));
}
