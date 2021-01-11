#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize,
        y: usize
    };
    let big = x.max(y);
    let small = x.min(y);
    println!("{}", ["No", "Yes"][(small + 3 > big) as usize]);
}
