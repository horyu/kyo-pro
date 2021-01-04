#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize
    };
    println!(
        "{}",
        (b - 1) / (a - 1) + (((b - 1) % (a - 1)) != 0) as usize
    );
}
