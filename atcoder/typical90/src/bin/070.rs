#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut xxyy: [(isize, isize); n]
    };
    xxyy.sort_unstable_by_key(|xy| xy.0);
    let cx = if n.is_odd() {
        xxyy[n / 2].0
    } else {
        (xxyy[n / 2 - 1].0 + xxyy[n / 2].0) / 2
    };
    xxyy.sort_unstable_by_key(|xy| xy.1);
    let cy = if n.is_odd() {
        xxyy[n / 2].1
    } else {
        (xxyy[n / 2 - 1].1 + xxyy[n / 2].1) / 2
    };
    let mut rs = 0;
    for (x, y) in xxyy {
        rs += (x - cx).abs() + (y - cy).abs();
    }
    println!("{rs}");
}
