#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        t: usize
    };
    println!("{}", b * (t / a));
}
