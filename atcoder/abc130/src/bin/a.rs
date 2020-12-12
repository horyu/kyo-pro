#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize,
        a: usize
    };
    println!("{}", [10, 0][(x < a) as usize]);
}
