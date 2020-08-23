#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        x: usize,
        t: usize
    };
    println!("{}", (n / x + (n % x != 0) as usize) * t);
}
