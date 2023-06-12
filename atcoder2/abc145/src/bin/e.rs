#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        t: usize,
        mut aabb: [(usize, isize); n],
    };
    aabb.sort_unstable();

    let mut dp = vec![isize::MIN; 6001];
    dp[0] = 0;
    for (a, b) in aabb.iter().copied() {
        let mut new_dp = dp.clone();
        for i in 0..t {
            new_dp[i + a] = new_dp[i + a].max(dp[i] + b);
        }
        dp = new_dp;
    }
    let rs = dp.iter().max().unwrap();
    println!("{rs}");
}
