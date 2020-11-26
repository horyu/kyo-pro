#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    };
    println!("{}", h * w + a * b - a * w - h * b);
}
