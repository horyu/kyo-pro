#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
        k: usize,
        s: Usize1,
        t: Usize1,
        x: Usize1,
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // dp[i][j] = i(xに訪れた回数 % 2)のときの頂点jにいるときの通り数
    let mut dp = vec![vec![ModInt998244353::default(); n]; 2];
    dp[0][s] += 1;
    for kk in 0..k {
        let mut new_dp = vec![vec![ModInt998244353::default(); n]; 2];
        for (dx, dp) in dp.into_iter().enumerate() {
            for (i, d) in dp.into_iter().enumerate() {
                for j in g[i].iter().copied() {
                    new_dp[dx ^ usize::from(j == x)][j] += d;
                }
            }
        }
        dp = new_dp;
    }
    let rs = dp[0][t];
    println!("{rs}");
}
