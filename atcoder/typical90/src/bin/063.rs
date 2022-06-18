#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        ppp: [[usize; w]; h]
    };
    let mut rs = 0usize;
    for ii in (1..=h).flat_map(|size| (0..h).combinations(size)) {
        let mut vv = vec![0; h * w + 1];
        for j in 0..w {
            if ii.iter().map(|&i| ppp[i][j]).all_equal() {
                vv[ppp[ii[0]][j]] += ii.len();
            }
        }
        rs = rs.max(vv.into_iter().max().unwrap_or_default());
    }
    println!("{rs}");
}
