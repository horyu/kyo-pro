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
        h: usize,
        w: usize,
        aaa: [Chars; h],
    };
    let bbb = aaa
        .iter()
        .map(|aa| {
            aa.iter()
                .map(|&a| [-1, 1][(a == '+') as usize])
                .collect_vec()
        })
        .collect_vec();
    let mut dp = vec![vec![-(1 << 29); w]; h];
    dp[h - 1][w - 1] = 0;
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i + 1 < h {
                dp[i][j] = dp[i][j].max(-dp[i + 1][j] + bbb[i + 1][j]);
            }
            if j + 1 < w {
                dp[i][j] = dp[i][j].max(-dp[i][j + 1] + bbb[i][j + 1]);
            }
        }
    }
    let rs = match 0.cmp(&dp[0][0]) {
        Ordering::Equal => "Draw",
        Ordering::Less => "Takahashi",
        Ordering::Greater => "Aoki",
    };
    println!("{rs}");
}
