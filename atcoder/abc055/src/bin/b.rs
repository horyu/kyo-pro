#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    println!("{}", (1..=n).fold(1, |rs, i| rs * i % 1_000_000_007));
}
