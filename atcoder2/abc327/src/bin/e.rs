#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use num_traits::Pow;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        pp: [f64; n],
    };
    // Rate(k) = sum{i=1..k}{0.9^(k-i) * Qi} / sum{i=1..k}{0.9^(k-i)} - 1200 / sqrt(k)
    // 分子だけを考えて、最後に計算する
    let mut dp = vec![0.0f64; n + 1];
    for (i, p) in pp.iter().copied().enumerate() {
        for j in (0..=i).rev() {
            dp[j + 1] = dp[j + 1].max(dp[j] * 0.9 + p);
        }
    }
    let mut rs = f64::MIN;
    for (k, bunsi) in dp.iter().copied().enumerate().skip(1) {
        let bunbo = (1.0 - 0.9f64.powi(k as i32)) / (1.0 - 0.9);
        rs = rs.max(bunsi / bunbo - 1200.0 / (k as f64).sqrt());
    }

    println!("{rs}");
}
