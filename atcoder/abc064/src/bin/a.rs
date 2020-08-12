#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _r: usize,
        g: usize,
        b: usize,
    };
    println!("{}", ["NO", "YES"][((g * 10 + b) % 4 == 0) as usize]);
}
