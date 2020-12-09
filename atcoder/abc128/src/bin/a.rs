#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        p: usize
    };
    println!("{}", (3 * a + p) / 2);
}
