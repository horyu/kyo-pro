#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        w: usize,
        h: usize
    };
    println!("{}", if w * 3 == h * 4 { "4:3" } else { "16:9" });
}
