#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_roundings)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        llrr: [(usize, usize); n]
    };
    let mut ll = 0;
    let mut rr = std::usize::MAX;
    for (l, r) in llrr {
        ll = ll.max(l);
        rr = rr.min(r);
        let rs = if ll <= rr {
            0
        } else {
            (ll - rr).div_ceil(2)
        };
        println!("{rs}");
    }
}
