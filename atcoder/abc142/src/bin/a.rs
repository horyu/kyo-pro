#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    println!("{}", (n + ((n % 2 != 0) as usize)) as f64 / 2.0 / n as f64);
}
