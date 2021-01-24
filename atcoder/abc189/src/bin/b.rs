#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        x: usize,
        vvpp: [(usize, usize); n]
    };
    let x = x * 100;
    let mut sum = 0;
    for (i, (v, p)) in vvpp.into_iter().enumerate() {
        sum += v * p;
        if sum > x {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
