#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        x: usize,
        y: usize
    };
    println!("{}", std::cmp::max(x, y));
}
