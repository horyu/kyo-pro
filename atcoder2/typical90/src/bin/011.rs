#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut ddccss: [(usize, usize, usize); n],
    };
    // ddccss.retain(|dcs| dcs.1 <= dcs.0);
    ddccss.sort_unstable_by_key(|dcs| dcs.0);
    // dp[見終わった仕事][合計仕事時間]
    let mut dp = vec![vec![0; 5001]; n + 1];
    for i in 0..n {
        let (d, c, s) = ddccss[i];
        for j in 0..5001 {
            if j < c || d < j {
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = dp[i][j].max(dp[i][j - c] + s);
            }
        }
    }
    let rs = dp[n].iter().max().copied().unwrap();
    println!("{rs}");
}
