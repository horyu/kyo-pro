#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize
    };
    println!("{}", [1, 0][(n % k == 0) as usize]);
}
