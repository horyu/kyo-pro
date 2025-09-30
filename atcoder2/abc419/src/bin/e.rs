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

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        aa: [usize; n],
    };
    // https://atcoder.jp/contests/abc419/editorial/13669
    let mut dp = vec![1usize << 60; m];
    dp[0] = 0;
    for i in 0..l {
        let mut new_dp = vec![1usize << 60; m];
        for j in 0..m {
            let mut cost = 0;
            for k in (i..n).step_by(l) {
                if aa[k] <= j {
                    cost += j - aa[k];
                } else {
                    cost += m + j - aa[k];
                }
            }
            for k in 0..m {
                new_dp[(j + k) % m] = new_dp[(j + k) % m].min(dp[k] + cost);
            }
        }
        dp = new_dp;
    }
    let rs = dp[0];
    println!("{rs}");
}
