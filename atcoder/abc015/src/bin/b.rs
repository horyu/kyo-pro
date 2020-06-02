#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let sum: usize = aa.iter().sum();
    let num = aa.iter().filter(|&&a| a != 0usize).count();
    println!("{}", ((sum as f64) / (num as f64)).ceil());
}
