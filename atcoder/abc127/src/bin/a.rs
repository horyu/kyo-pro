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
        if a <= 5 {
            0
        } else if a <= 12 {
            b / 2
        } else {
            b
        }
    );
}
