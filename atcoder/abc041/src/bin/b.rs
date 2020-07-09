#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        abc: [usize; 3],
    };
    println!(
        "{}",
        abc.iter().fold(1, |acc, v| (acc * v) % (1e9 as usize + 7))
    );
}
