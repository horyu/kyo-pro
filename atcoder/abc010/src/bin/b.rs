#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut sum = 0;
    // 1 2 3 4 5 0
    // o x o x o x
    // o x o o x o
    for a in aa.iter() {
        sum += match a % 6 {
            1 => 0,
            2 => 1,
            3 => 0,
            4 => 1,
            5 => 2,
            _ => 3
        };
    }
    println!("{}", sum);
}
