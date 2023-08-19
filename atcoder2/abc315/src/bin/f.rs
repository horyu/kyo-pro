#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        xxyy: [(f64, f64); n],
    };
    let mut dd = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            dd[i][j] = ((xxyy[i].0 - xxyy[j].0).powi(2) + (xxyy[i].1 - xxyy[j].1).powi(2)).sqrt();
        }
    }
    let mut dp = vec![[1e10f64; 60]; n];
    dp[0][0] = 0.0;
    for i in 0..(n - 1) {
        for j in 0..30 {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + dd[i][i + 1]);
            for k in 2..30 {
                if i + k < n {
                    dp[i + k][j + k - 1] = dp[i + k][j + k - 1].min(dp[i][j] + dd[i][i + k]);
                }
            }
        }
    }
    let mut rs = f64::MAX;
    for (i, d) in dp[n - 1].into_iter().enumerate() {
        if i == 0 {
            rs = rs.min(d);
        } else {
            rs = rs.min(d + 2.0f64.powi(i as i32 - 1));
        }
    }
    println!("{rs}");
}
