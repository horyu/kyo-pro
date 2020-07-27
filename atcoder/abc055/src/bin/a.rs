#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    println!("{}", 800 * n - 200 * (n / 15));
}
