#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        h: usize
    };
    println!("{}", (a + b) * h / 2);
}
