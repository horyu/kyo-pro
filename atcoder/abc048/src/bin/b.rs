#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize
    };
    let right = b / x;
    let left = if a % x == 0 { a / x } else { a / x + 1 };
    println!("{}", 1 + right - left);
}
