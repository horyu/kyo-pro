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
        w: usize,
        n: usize,
        k: usize,
        aabb: [(usize, usize); n],
    };
    let mut rs = 0;
    let mut dp = vec![vec![0usize; 10001]; k + 1];
    for (a, b) in aabb.iter().copied() {
        for i in (0..k).rev() {
            for j in 0..10000 {
                let v = dp[i][j];
                let new_width = j + a;
                if (i == 0 || v != 0) && new_width <= w {
                    let new_worth = v + b;
                    rs = rs.max(new_worth);
                    dp[i + 1][new_width] = new_worth.max(dp[i + 1][new_width]);
                }
            }
        }
    }
    println!("{rs}");
}
