#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        aa: [usize; h * w]
    };
    let min = aa.iter().min().unwrap();
    let rs = aa.iter().fold(0, |acc, a| acc + a - min);
    println!("{}", rs);
}
