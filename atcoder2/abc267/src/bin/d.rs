#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        m: usize,
        aa: [isize; n],
    };
    let mut dp = vec![aa[0]];
    for i in 1..n {
        let mut new_dp = vec![std::isize::MIN; (i + 1).min(m)];
        for j in 1..=i.min(m - 1) {
            if let Some(&pre) = dp.get(j) {
                new_dp[j] = new_dp[j].max(pre);
            }
            // eprintln!("{i} {j} {}", i.min(m));
            new_dp[j] = new_dp[j].max(dp[j - 1] + (j + 1) as isize * aa[i]);
        }
        new_dp[0] = dp[0].max(aa[i]);
        dp = new_dp;
    }
    let rs = dp[m - 1];
    println!("{rs}");
}
