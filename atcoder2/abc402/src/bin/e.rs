#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        ssccpp: [(f64, usize, f64); n],
    };
    // https://atcoder.jp/contests/abc402/editorial/12715
    // dp[t][j] := tに含まれる問題を正解していて所持金の残りがj円であるときの期待値
    let mut dp = vec![vec![0.0; x + 1]; 1 << n];
    for x in 1..=x {
        for t in 0..(1 << n) {
            for i in 0..n {
                let (s, c, p) = ssccpp[i];
                if x < c || 0 != (t & (1 << i)) {
                    continue;
                }
                let p = p / 100.0;
                let xx = x - c;
                let tt = t | (1 << i);
                let v = p * (s + dp[tt][xx]) + (1.0f64 - p) * dp[t][xx];
                dp[t][x] = dp[t][x].max(v);
            }
        }
    }
    let rs = dp[0][x];
    println!("{rs}");
}
