#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };
    println!("{}", (c + b).checked_sub(a).unwrap_or(0));
}
