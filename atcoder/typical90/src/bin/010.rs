#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        ccpp: [(Usize1, usize); n],
        q: usize,
        llrr: [(usize, usize); q]
    };
    let mut vvv = vec![vec![0; n + 1];2 ];
    for (i, (c, p)) in ccpp.into_iter().enumerate() {
        for vv in vvv.iter_mut() {
            vv[i + 1] = vv[i];
        }
        vvv[c][i + 1] += p;
    }
    for (l, r) in llrr {
        println!("{} {}", vvv[0][r] - vvv[0][l - 1], vvv[1][r] - vvv[1][l - 1],);
    }
}
