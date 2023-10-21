#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use num_traits::Signed;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        xxyyzz: [(isize, isize, isize); n],
    };
    // bitDP
    // https://algo-logic.info/bit-dp/
    let dd = xxyyzz
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| {
            xxyyzz
                .iter()
                .copied()
                .map(|q| p.0.abs_diff(q.0) + p.1.abs_diff(q.1) + 0.max(p.2 - q.2) as usize)
                .collect_vec()
        })
        .collect_vec();
    let mut dp = vec![vec![!0; n]; 1 << n];
    dp[0][0] = 0;
    let rs = f(n, &mut dp, &dd, (1 << n) - 1, 0);
    println!("{rs}");
}

fn f(n: usize, dp: &mut [Vec<usize>], dd: &[Vec<usize>], s: usize, j: usize) -> usize {
    if s == 0 {
        if j == 0 {
            return 0;
        }
        return !0;
    }
    if s & (1 << j) == 0 {
        return !0;
    }
    if dp[s][j] != !0 {
        return dp[s][j];
    }
    for i in 0..n {
        dp[s][j] = dp[s][j].min(f(n, dp, dd, s ^ (1 << j), i).saturating_add(dd[i][j]));
    }

    dp[s][j]
}
