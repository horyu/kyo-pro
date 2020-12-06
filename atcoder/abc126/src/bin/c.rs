#![allow(unused_imports)]
// use itertools::Itertools;
use std::collections::btree_map::Keys;

use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize
    };
    let base = 1.0 / (n as f64);
    let rs = (1usize..=n).fold(0f64, |acc, i| {
        if i >= k {
            acc + base
        } else {
            let mut score = i;
            let mut p = base;
            while score < k {
                score *= 2;
                p /= 2f64;
            }
            acc + p
        }
    });
    println!("{}", rs);
}
