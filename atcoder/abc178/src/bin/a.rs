#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize
    };
    println!("{}", [0, 1][(x == 0) as usize]);
}
