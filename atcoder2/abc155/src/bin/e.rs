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
        n: Bytes,
    };
    let bb = n.iter().map(|&b| (b - b'0') as usize).collect::<Vec<_>>();
    // i桁目まで [ちょうど, 超過] はらった時の双方が使用する紙幣の最小枚数
    let mut dp = vec![[1usize << 60; 2]; n.len() + 1];
    dp[0] = [0, 1];
    for (i, b) in bb.iter().copied().enumerate() {
        dp[i + 1][0] = (dp[i][0] + b).min(dp[i][1] + 10 - b);
        dp[i + 1][1] = (dp[i][0] + b + 1).min(dp[i][1] + 10 - b - 1);
    }
    let rs = dp[n.len()][0];
    println!("{rs}");
}
