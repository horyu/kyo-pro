#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h1: usize,
        w1: usize,
        h2: usize,
        w2: usize
    };
    if (h1 == h2) || (h1 == w2) || (w1 == h2) || (w1 == w2) {
        println!("YES");
    } else {
        println!("NO");
    }
}
