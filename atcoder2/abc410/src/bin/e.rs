#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        h: usize,
        m: usize,
        aabb: [(usize, usize); n],
    };
    // https://atcoder.jp/contests/abc410/editorial/13207
    // dp[i + 1][m] = i番目のモンスターを倒した時点で魔力がmであるときの最大h
    let mut dp = vec![vec![!0; m + 1]; n + 1];
    dp[0][m] = h;
    for (i, (a, b)) in aabb.into_iter().enumerate() {
        for j in 0..=m {
            if dp[i][j] == !0 {
                continue;
            }
            if a <= dp[i][j] {
                if dp[i + 1][j] == !0 {
                    dp[i + 1][j] = dp[i][j] - a;
                } else {
                    dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] - a);
                }
            }
            if b <= j {
                if dp[i + 1][j - b] == !0 {
                    dp[i + 1][j - b] = dp[i][j];
                } else {
                    dp[i + 1][j - b] = dp[i + 1][j - b].max(dp[i][j]);
                }
            }
        }
    }
    let rs = dp
        .into_iter()
        .rposition(|dp| dp.into_iter().any(|d| d != !0))
        .unwrap_or_default();
    println!("{rs}");
}
