#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    let mut min = std::isize::MAX;
    let mut x = 0isize;
    let mut y: isize = aa.iter().sum();
    for i in 0..(n - 1) {
        let a = aa[i];
        x += a;
        y -= a;
        min = std::cmp::min(min, (x - y).abs());
    }
    println!("{}", min);
}
