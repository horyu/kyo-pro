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
        ttxxaa: [(usize, usize, usize); n],
    };
    let mut ttxxaa: VecDeque<_> = ttxxaa.into_iter().collect();
    let mut dp = [0; 5];
    for tt in 0..=100000 {
        if let Some((t, x, a)) = ttxxaa.front().copied() {
            if t == tt {
                ttxxaa.pop_front();
                if x <= tt {
                    dp[x] += a;
                    // eprintln!("[{tt}] {}", dp.iter().join(" "));
                }
            }
        }
        let mut new_dp = dp;
        for i in 0..5 {
            if 0 < i {
                new_dp[i] = new_dp[i].max(dp[i - 1]);
            }
            if i < 4 {
                new_dp[i] = new_dp[i].max(dp[i + 1]);
            }
        }
        dp = new_dp;
    }
    // eprintln!("[] {}", dp.iter().join(" "));
    let rs = dp.into_iter().max().unwrap();
    println!("{rs}");
}
