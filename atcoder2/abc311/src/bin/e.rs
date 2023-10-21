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
        h: usize,
        w: usize,
        n: usize,
        aabb: [(usize, usize); n],
    };
    const MAX: usize = 3003;
    let mut ng = vec![vec![false; MAX]; MAX];
    for (a, b) in aabb {
        ng[a][b] = true;
    }
    let mut dp = vec![vec![0usize; MAX]; MAX];
    let mut rs = 0usize;
    for i in 1..=h {
        for j in 1..=w {
            if ng[i][j] {
                continue;
            }
            dp[i][j] = dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j]) + 1;
            rs += dp[i][j];
        }
    }
    // for i in 1..=h {
    //     eprintln!("{}", (1..=w).map(|j| format!("{:2}", dp[i][j])).join(" "));
    // }
    println!("{rs}");
}
