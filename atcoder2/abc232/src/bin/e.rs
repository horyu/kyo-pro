#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use ac_library_rs::ModInt998244353;
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        x1: Usize1,
        y1: Usize1,
        x2: Usize1,
        y2: Usize1,
    };
    let mut dp = [ModInt998244353::new(0); 4];
    dp[match (x1 == x2, y1 == y2) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    }] += 1;
    for i in 0..k {
        let mut new_dp = [ModInt998244353::new(0); 4];
        new_dp[0] += dp[1] + dp[2];
        new_dp[1] += dp[0] * (w - 1) + dp[1] * (w - 2) + dp[3];
        new_dp[2] += dp[0] * (h - 1) + dp[2] * (h - 2) + dp[3];
        new_dp[3] += dp[1] * (h - 1) + dp[2] * (w - 1) + dp[3] * (w + h - 4);
        dp = new_dp;
    }
    let rs = dp[0];
    println!("{rs}");
}
