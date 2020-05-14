#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: f64
    };
    println!("{}", 10000.0 / 2.0 * (n + 1.0));
}
