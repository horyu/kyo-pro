#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut llrr: [(usize, usize); n]
    };
    llrr.sort_unstable_by_key(|lr| lr.0);
    let mut vv = vec![];
    let (mut x, mut y) = llrr[0];
    for &(l, r) in &llrr[1..] {
        if l <= y {
            y = y.max(r);
        } else {
            vv.push((x, y));
            x = l;
            y = r;
        }
    }
    vv.push((x, y));
    for (l, r) in vv {
        println!("{l} {r}");
    }
}
