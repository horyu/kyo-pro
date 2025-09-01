#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        h: usize,
        w: usize,
        aaa: [[usize; w]; h],
        pp: [usize; h + w - 1],
    };
    // https://atcoder.jp/contests/abc415/editorial/13490
    let mut dp = vec![vec![1e18 as usize; w]; h];
    dp[h - 1][w - 1] = 0;
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i + 1 < h {
                dp[i][j] = dp[i][j].min(dp[i + 1][j]);
            }
            if j + 1 < w {
                dp[i][j] = dp[i][j].min(dp[i][j + 1]);
            }
            dp[i][j] = (dp[i][j] + pp[i + j]).saturating_sub(aaa[i][j]);
        }
    }
    let rs = dp[0][0];
    println!("{rs}");
}
