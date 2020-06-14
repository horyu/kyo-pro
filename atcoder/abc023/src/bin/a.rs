#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize
    };
    println!("{}", x / 10 + x % 10);
}
