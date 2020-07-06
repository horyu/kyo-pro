#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };
    println!("{}", 2 * (a * b + b * c + c * a));
}
