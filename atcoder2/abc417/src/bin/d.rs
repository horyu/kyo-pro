#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        ppaabb: [(usize, usize, usize); n],
        q: usize,
        xx: [usize; q],
    };
    // テンション0..=1000の範囲で逆順にDP
    // dp[i][j] = テンションjでi番目のプレゼントを処理したときの最終的な値
    let mut memo = vec![vec![None::<usize>; 1001]; n + 1];
    memo[n] = (0..=1000).map(Some).collect_vec();
    let mut subs = vec![0; n + 1];
    for (i, (p, a, b)) in ppaabb.iter().copied().enumerate() {
        subs[i + 1] = subs[i] + b;
    }
    for x in xx {
        let pos = subs.partition_point(|&s| 500 + s < x);
        // TODO
        // 処理しつつmemoに値があればそれを使う
        // memoを更新
    }
}
