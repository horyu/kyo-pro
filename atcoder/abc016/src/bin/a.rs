#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        m: usize,
        d: usize
    };
    println!("{}", if m % d == 0 { "YES" } else { "NO" });
}
