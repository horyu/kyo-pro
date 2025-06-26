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
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            aa: [usize; 2 * n],
        };
        // dp[i] := (括弧がi個ある時のMAX
        let mut dp = vec![0; 1];
        for (i, a) in aa.iter().copied().enumerate() {
            let mut new_dp = vec![0; dp.len() + 1];
            for (j, v) in dp.iter().copied().enumerate().rev() {
                new_dp[j + 1] = new_dp[j + 1].max(dp[j] + a);
                if 0 < j {
                    new_dp[j - 1] = new_dp[j - 1].max(dp[j]);
                }
            }
            new_dp.truncate((n + 1).min(2 * n - i + 1));
            dp = new_dp;
        }
        let rs = dp[0];
        println!("{rs}");
    }
}
