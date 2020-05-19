#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    println!("{}", if n % 3 == 0 { "YES" } else { "NO" });
}
