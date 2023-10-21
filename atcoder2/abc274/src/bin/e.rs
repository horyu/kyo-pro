#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        // xxyy: [(isize, isize); n],
        // ppqq: [(isize, isize); m],
        xxyy: [(f64, f64); n + m],
    };
    // bit dp
    let nm = n + m;
    let mut dp = vec![vec![1e60; nm]; 1 << nm];
    let mut dd = vec![vec![1e60; nm + 1]; nm + 1];
    let len = |i: usize, j: usize| -> f64 {
        let (xi, yi) = xxyy[i];
        let (xj, yj) = xxyy.get(j).copied().unwrap_or_default();
        ((xi - xj).powi(2) + (yi - yj).powi(2)).sqrt()
    };
    for i in 0..nm {
        let (xi, yi) = xxyy[i];
        for j in (i + 1)..nm {
            let (xj, yj) = xxyy[j];
            dd[i][j] = len(i, j);
            dd[j][i] = dd[i][j];
        }
        dd[i][nm] = len(i, nm);
        dp[1 << i][i] = dd[i][nm];
    }
    let mask: usize = (1 << n) - 1;
    for s in 1usize..(1 << nm) {
        for v in 0..nm {
            for u in 0..nm {
                if 0 == (s & (1 << u)) {
                    continue;
                }
                if 0 == (s & (1 << v)) {
                    let div = 2.0f64.powi((s >> n).count_ones() as i32);
                    dp[s | (1 << v)][v] = dp[s | (1 << v)][v].min(dp[s][u] + dd[u][v] / div);
                }
            }
        }
    }
    let mut rs = 1e66f64;
    for mm in 0..(1 << m) {
        let s = mask | (mm << n);
        for i in 0..nm {
            if 0 != s & (1 << i) {
                let div = 2.0f64.powi(mm.count_ones() as i32);
                rs = rs.min(dp[s][i] + dd[i][nm] / div);
                // eprintln!("{s:4b} {i} {div} {}", dp[s][i]);
            }
        }
    }
    println!("{rs}");
}
