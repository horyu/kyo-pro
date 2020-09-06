#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize
    };
    let mut i = 1;
    for _ in 0..n {
        i = std::cmp::min(i * 2, i + k);
    }
    println!("{}", i);
}
