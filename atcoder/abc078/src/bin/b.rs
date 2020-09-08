#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize
    };
    println!("{}", (x - z) / (y + z));
}
