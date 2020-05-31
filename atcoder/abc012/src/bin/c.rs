#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let k = 2025 - n;
    for i in 1..=9 {
        for j in 1..=9 {
            if k == i * j {
                println!("{} x {}", i, j);
            }
        }
    }
}
