#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        ab: [usize; 2],
        c: usize,
    };
    println!("{}", c / ab.iter().min().unwrap());
}
