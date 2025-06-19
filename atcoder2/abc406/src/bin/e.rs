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
        nnkk: [(usize, usize); t],
    };
    for (n, k) in nnkk {
        let size = n.ilog2() as usize + 1;
        // dp[i][b][j] = i桁目までオンビットの数がj個となる数
        let mut dp = vec![vec![vec![0; k + 1]; 2]; size];
        dp[0][0][0] = 1; // 0 bits,
        for i in 0..size - 1 {
            for b in 0..2 {
                for j in 0..=k {
                    if dp[i][b][j] == 0 {
                        continue;
                    }
                    // i桁目を0にする
                    dp[i + 1][b][j] += dp[i][b][j];
                    // i桁目を1にする
                    if b == 0 && j + 1 <= k {
                        dp[i + 1][1][j + 1] += dp[i][b][j];
                    }
                }
            }
        }
    }
    // println!("{rs}");
}
