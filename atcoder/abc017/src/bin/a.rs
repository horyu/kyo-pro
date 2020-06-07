#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        ssee: [(usize, usize); 3]
    };
    println!("{}", ssee.iter().fold(0, |sum, (s, e)| sum + s * e) / 10);
}
