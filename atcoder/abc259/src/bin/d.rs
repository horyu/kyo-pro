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
        ss: (isize, isize),
        tt: (isize, isize),
        xxyyrr: [(isize, isize, isize); n],
    };
    let ssii = (0..n)
        .filter(|&i| {
            let (x, y, r) = xxyyrr[i];
            (ss.0 - x).pow(2u32) + (ss.1 - y).pow(2u32) == r * r
        })
        .collect_vec();
    let ttii = (0..n)
        .filter(|&i| {
            let (x, y, r) = xxyyrr[i];
            (tt.0 - x).pow(2u32) + (tt.1 - y).pow(2u32) == r * r
        })
        .collect_vec();

    let mut uf = UnionFind::new(n);
    for (i, j) in (0..n).tuple_combinations() {
        let (xi, yi, ri) = xxyyrr[i];
        let (xj, yj, rj) = xxyyrr[j];
        let dd = (xi - xj).pow(2u32) + (yi - yj).pow(2u32);
        if ((ri - rj).pow(2u32)..=((ri + rj).pow(2u32))).contains(&dd) {
            uf.union(i, j);
        }
    }

    for &si in &ssii {
        for &ti in &ttii {
            if uf.equiv(si, ti) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
