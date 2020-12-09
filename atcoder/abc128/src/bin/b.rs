#![allow(unused_imports)]
// use itertools::Itertools;
use std::vec;

use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
    };
    let mut spsp = vec![];
    for i in 1..=n {
        input! {
            s: String,
            p: usize,
        };
        spsp.push((i, s, p));
    }
    spsp.sort_by(|a, b| {
        use std::cmp::Ordering;
        match a.1.cmp(&b.1) {
            Ordering::Equal => b.2.cmp(&a.2),
            other => other,
        }
    });
    for (i, _s, _p) in spsp {
        println!("{}", i);
    }
}
