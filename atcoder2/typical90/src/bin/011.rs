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
        mut ddccss: [(usize, usize, usize); n],
    };
    ddccss.sort_unstable();
    const MAX: usize = 5000;
    let mut dp = vec![0; MAX + 1];
    for (d, c, s) in ddccss.iter().copied() {
        if d < c {
            continue;
        }
        let mut new_dp = dp.clone();
        for i in 0..=(d - c) {
            new_dp[i + c] = new_dp[i + c].max(dp[i] + s);
        }
        dp = new_dp;
        // eprintln!("{:?}", &dp[..9]);
    }
    let rs = dp.iter().max().unwrap();
    println!("{rs}");
}
