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
use std::collections::{
    hash_map::RandomState, BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    let mut dp = vec![ModInt998244353::new(0); n];
    dp[0] += 1;
    for _ in 0..k {
        let mut new_dp = vec![ModInt998244353::new(0); n];
        let sum = dp.iter().fold(ModInt998244353::new(0), |acc, v| acc + v);
        for i in 0..n {
            new_dp[i] = sum - dp[i];
            for j in g[i].iter().copied() {
                new_dp[i] -= dp[j];
            }
        }
        dp = new_dp;
    }
    let rs = dp[0];
    println!("{rs}");
}
