#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    println!("{}", aa.iter().fold(0, |acc, a| acc + a - 1));
}
