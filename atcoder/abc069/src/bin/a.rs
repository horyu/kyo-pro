#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize,
        m: isize
    };
    println!("{}", (n - 1) * (m - 1));
}
