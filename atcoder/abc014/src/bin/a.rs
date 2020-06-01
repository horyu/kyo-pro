#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize
    };
    println!(
        "{}",
        if a % b == 0 {
            (a / b) * b - a
        } else {
            (1 + a / b) * b - a
        }
    );
}
