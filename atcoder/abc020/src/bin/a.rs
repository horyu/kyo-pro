#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        q: usize
    };
    println!("{}", if q == 1 { "ABC" } else { "chokudai" });
}
