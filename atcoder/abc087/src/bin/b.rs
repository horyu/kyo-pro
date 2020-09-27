#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    };
    let mut count = 0;
    for s in 0..=a {
        for t in 0..=b {
            for u in 0..=c {
                if 500 * s + 100 * t + 50 * u == x {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
