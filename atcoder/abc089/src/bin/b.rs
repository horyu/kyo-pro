#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ss: [char; n]
    };
    println!("{}", ["Three", "Four"][ss.contains(&'Y') as usize]);
}
