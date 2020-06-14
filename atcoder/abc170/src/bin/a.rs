#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        xx: [usize; 5]
    };
    println!("{}", xx.iter().position(|&x| x == 0).unwrap() + 1);
}
