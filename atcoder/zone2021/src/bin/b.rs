#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        d0: isize,
        h0: isize,
        dhdh: [(isize, isize); n]
    };
    if dhdh.iter().any(|(_d, h)| *h == h0) {
        println!("{}", h0);
        return;
    }
    let max = dhdh
        .into_iter()
        .map(|(d, h)| {
            if d == d0 {
                0.0
            } else {
                (-1.0) * (d0 as f64) * ((h0 - h) as f64 / (d0 - d) as f64) + (h0 as f64)
            }
        })
        .fold(0.0, f64::max);
    println!("{}", max);
}
