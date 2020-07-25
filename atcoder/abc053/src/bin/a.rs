#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize
    };
    println!("{}", if x < 1200 { "ABC" } else { "ARC" });
}
