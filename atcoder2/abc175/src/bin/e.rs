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
        r: usize,
        c: usize,
        k: usize,
        rrccvv: [(usize, usize, isize); k],
    };
    let mut hm = HashMap::new();
    for (r, c, v) in rrccvv {
        hm.insert((r, c), v);
    }
    let mut dp = vec![vec![[0isize; 4]; c + 1]; r + 1];
    for y in 1..=r {
        for x in 1..=c {
            dp[y][x] = dp[y][x - 1];
            dp[y][x][0] = dp[y][x][0].max(dp[y - 1][x].iter().max().copied().unwrap());

            if let Some(v) = hm.get(&(y, x)).copied() {
                for kk in (0..3).rev() {
                    dp[y][x][kk + 1] = dp[y][x][kk + 1].max(dp[y][x][kk] + v);
                }
            }
        }
    }
    let rs = dp[r][c].iter().max().unwrap();
    println!("{rs}");
}
