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
        s: Bytes,
    };
    // https://atcoder.jp/contests/abc215/editorial/2483
    let s = s.into_iter().map(|b| (b - b'A') as usize).collect_vec();
    let mut dp = vec![vec![[ModInt998244353::default(); 10]; 1024]; n + 1];
    for (i, b) in s.iter().copied().enumerate() {
        for u in 0..1024 {
            for j in 0..10 {
                let tmp = dp[i][u][j];
                dp[i + 1][u][j] = tmp;
                if j == b {
                    dp[i + 1][u][j] += tmp;
                }
            }
        }
        for v in 0..1024 {
            if 0 == (v & (1 << b)) {
                for j in 0..10 {
                    let tmp = dp[i][v][j];
                    dp[i + 1][v | (1 << b)][b] += tmp;
                }
            }
        }
        dp[i + 1][1 << b][b] += 1;
    }
    let mut rs = ModInt998244353::default();
    for u in 0..1024 {
        for j in 0..10 {
            rs += dp[n][u][j];
        }
    }
    println!("{rs}");
}
