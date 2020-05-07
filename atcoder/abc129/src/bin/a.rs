#![allow(unused_imports)]
use ascii::{AsciiChar, AsciiStr};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        p: usize,
        q: usize,
        r: usize
    };
    println!("{}", vec![p + q, q + r, r + p].into_iter().min().unwrap());
}
