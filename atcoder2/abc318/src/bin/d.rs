#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
#![allow(dead_code)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use pathfinding::directed::dfs;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
    };
    let mut ddd = vec![vec![0; n]; n];
    for i in 0..(n - 1) {
        input! {
            dd: [usize; n - i - 1],
        };
        for (j, d) in dd.into_iter().enumerate() {
            ddd[i][i + j + 1] = d;
            ddd[i + j + 1][i] = d;
        }
    }
    let mut dp = vec![0usize; 1 << n];
    for b in 0..(1 << n) {
        let mut l = !0;
        for i in 0..n {
            if b & (1 << i) == 0 {
                l = i;
                break;
            }
        }
        for i in 0..n {
            if b & (1 << i) == 0 {
                let nb = b | (1 << l) | (1 << i);
                dp[nb] = dp[nb].max(dp[b] + ddd[l][i]);
            }
        }
    }
    let rs = dp.last().unwrap();
    println!("{rs}");
}

fn main2() {
    input! {
        n: usize,
    };
    let mut ddd = vec![vec![0; n]; n];
    for i in 0..(n - 1) {
        input! {
            dd: [usize; n - i - 1],
        };
        for (j, d) in dd.into_iter().enumerate() {
            ddd[i][i + j + 1] = d;
            ddd[i + j + 1][i] = d;
        }
    }
    let mut rs = dfs(&mut vec![true; n], &ddd, 0);
    if n % 2 == 1 {
        for i in 0..n {
            let mut ttff = vec![true; n];
            ttff[i] = false;
            rs = rs.max(dfs(&mut ttff, &ddd, 0));
        }
    }
    println!("{rs}");
}

fn dfs(ttff: &mut Vec<bool>, ddd: &[Vec<usize>], sum: usize) -> usize {
    let n = ttff.len();
    let tt = ttff.iter().copied().positions(|tf| tf).collect_vec();
    if tt.len() <= 1 {
        return sum;
    }
    let mut rs = sum;
    let t0 = tt[0];
    ttff[t0] = false;
    for ti in tt[1..].iter().copied() {
        ttff[ti] = false;
        rs = rs.max(dfs(ttff, ddd, sum + ddd[t0][ti]));
        ttff[ti] = true;
    }
    ttff[t0] = true;
    rs
}
