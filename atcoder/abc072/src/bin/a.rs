#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: isize,
        t: isize
    };
    println!("{}", std::cmp::max(0, x - t));
}
