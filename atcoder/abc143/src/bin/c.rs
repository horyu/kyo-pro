#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        mut s: Chars
    };
    s.dedup();
    println!("{}", s.len());
}
