#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        lrlr: [(usize, usize); m]
    };
    let mut left = 0;
    let mut right = n;
    for (l, r) in lrlr {
        use std::cmp;
        left = cmp::max(l, left);
        right = cmp::min(r, right);
    }
    let rs = if right >= left { right - left + 1 } else { 0 };
    println!("{}", rs);
}
