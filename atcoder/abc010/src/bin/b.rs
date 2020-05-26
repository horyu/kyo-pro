#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    // 1 2 3 4 5 0
    // o x o x o x
    // o x o o x o
    let sum: usize = aa
        .iter()
        .map(|&a| match a % 6 {
            1 | 3 => 0,
            2 | 4 => 1,
            5 => 2,
            _ => 3,
        })
        .sum();
    println!("{}", sum);
}
