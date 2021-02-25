#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        xx: [usize; n],
    };
    let rs = (1..=100)
        .map(|p| xx.iter().fold(0, |acc, x| acc + x * x + p * p - 2 * p * x))
        .min()
        .unwrap();
    println!("{}", rs);
}
