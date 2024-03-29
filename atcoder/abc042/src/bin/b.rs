#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        _l: usize,
        mut ss: [String; n]
    };
    ss.sort_unstable();
    println!("{}", ss.iter().join(""));
}
