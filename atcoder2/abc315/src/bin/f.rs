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
        xxyy: [(usize, usize); n],
    };
    // スキップしたときのコストは　2^(スキップした回数) であるため、2^30 程度まで考えれば十分

    // ((x, y), skip_cnt) -> cost
    let mut hm = HashMap::new();
    hm.insert((xxyy[0], 0), 0.0);
    for (x, y) in xxyy[1..].iter().copied() {
        let mut new_hm = HashMap::new();
        for (((px, py), skip_cnt), move_cost) in hm {
            if skip_cnt < 30 {
                let e = new_hm.entry(((px, py), skip_cnt + 1)).or_insert(f64::MAX);
                *e = e.min(move_cost);
            }
            let new_cost =
                move_cost + ((x.abs_diff(px).pow(2) + y.abs_diff(py).pow(2)) as f64).sqrt();
            let e = new_hm.entry(((x, y), skip_cnt)).or_insert(f64::MAX);
            *e = e.min(new_cost);
        }
        hm = new_hm;
    }
    let mut rs = f64::MAX;
    for ((pos, skip_cnt), move_cost) in hm {
        if pos == xxyy[n - 1] {
            let skip_cost = if skip_cnt == 0 {
                0.0
            } else {
                2.0f64.powi(skip_cnt - 1)
            };
            rs = rs.min(move_cost + skip_cost);
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
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
