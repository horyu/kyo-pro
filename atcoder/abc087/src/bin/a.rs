#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize,
        a: usize,
        b: usize,
    };
    println!("{}", (x - a) % b);
}
