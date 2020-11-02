#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        k: usize
    };
    println!("{}", ((k + 1) / 2) * (k / 2));
}
