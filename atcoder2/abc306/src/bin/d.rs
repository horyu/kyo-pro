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
        xxyy: [(isize, isize); n]
    };
    let mut dp = [0, isize::MIN / 4];
    for (x, y) in xxyy {
        let mut new_dp = dp;
        if x == 0 {
            new_dp[0] = new_dp[0].max(dp[0] + y).max(dp[1] + y);
        } else {
            new_dp[1] = new_dp[1].max(dp[0] + y);
        }
        dp = new_dp;
    }
    let rs = dp.into_iter().max().unwrap();
    println!("{rs}");
}
