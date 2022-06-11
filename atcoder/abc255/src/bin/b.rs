#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [Usize1; k],
        xxyy: [(f64, f64); n]
    };
    let mut bb = vec![];
    let mut cc = vec![];
    for (i, (x, y)) in xxyy.into_iter().enumerate() {
        if aa.contains(&i) {
            bb.push((x, y));
        } else {
            cc.push((x, y));
        }
    }
    let mut rs = 0.0f64;
    for c in cc {
        let mut min = std::f64::MAX;
        for &b in &bb {
            min = min.min(((c.0 - b.0).powi(2i32) + (c.1 - b.1).powi(2i32)).sqrt());
        }
        rs = rs.max(min);
    }
    println!("{rs}");
}
