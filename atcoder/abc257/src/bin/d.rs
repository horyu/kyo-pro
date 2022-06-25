#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_roundings)]
use itertools::Itertools;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        xxyypp: [(isize, isize, usize); n],
    };
    let mut vvv = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            let xypi = xxyypp[i];
            let xypj = xxyypp[j];
            let diff = xypi.0.abs_diff(xypj.0) + xypi.1.abs_diff(xypj.1);
            vvv[i][j] = diff.div_ceil(xypi.2);
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let max = vvv[i][k].max(vvv[k][j]);
                if max < vvv[i][j] {
                    vvv[i][j] = max;
                }
            }
        }
    }
    let rs = vvv
        .into_iter()
        .map(|vv| vv.into_iter().max().unwrap())
        .min()
        .unwrap();
    println!("{rs}");
}
