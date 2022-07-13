#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        max: usize,
        ttxxyyvv: [(usize, Usize1, Usize1, isize); max],
    };
    let mut vv = vec![0isize; n];
    for &(t, _x, y, v) in &ttxxyyvv {
        if t == 0 {
            vv[y] = v;
        }
    }
    for i in 1..n {
        vv[i] -= vv[i - 1];
    }
    let mut uf = UnionFind::new(n);
    for (t, x, y, v) in ttxxyyvv {
        if t == 0 {
            uf.union(x, y);
            continue;
        }
        if uf.equiv(x, y) {
            let rs = if (x + y).is_even() {
                vv[y] + (v - vv[x])
            } else {
                vv[y] - (v - vv[x])
            };
            println!("{}", rs);
        } else {
            println!("Ambiguous");
        }
    }
}
