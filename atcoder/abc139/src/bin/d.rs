#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    println!("{}", (1..n).sum::<usize>());
}
