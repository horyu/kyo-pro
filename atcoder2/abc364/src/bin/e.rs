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
        x: usize,
        y: usize,
        aabb: [(usize, usize); n],
    };
    // https://atcoder.jp/contests/abc364/editorial/10550
    // dp[i][j][k] = 料理1..=iからj個選んで甘さがkになる最小のしょっぱさ
    let mut dp = vec![vec![vec![1usize << 60; x + 1]; n + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        let (a, b) = aabb[i];
        for j in 0..=i {
            for k in 0..=x {
                dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
                if k + a <= x {
                    dp[i + 1][j + 1][k + a] = dp[i + 1][j + 1][k + a].min(dp[i][j][k] + b);
                }
            }
        }
    }
    for p in (0..=n).rev() {
        for q in 0..=x {
            if dp[n][p][q] <= y {
                let rs = (p + 1).min(n);
                println!("{rs}");
                return;
            }
        }
    }
}
