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

/*
100 -> 00,11 -> 1,1
1100: [1][1],1[0]1 -> 1,01 -> 1,0
*/

fn main() {
    input! {
        n: usize,
        t: Chars,
    };
    // https://atcoder.jp/contests/abc418/editorial/13622
    // dp[i][j] = i文字目まで見た時、0を2m+j子含む部分文字列の個数
    let mut dp = vec![[0; 2]; n + 1];
    for (i, c) in t.iter().copied().enumerate() {
        if c == '0' {
            dp[i + 1] = [dp[i][1], dp[i][0] + 1];
        } else {
            dp[i + 1] = [dp[i][0] + 1, dp[i][1]];
        }
    }
    let rs = dp.into_iter().map(|d| d[0]).sum::<usize>();
    println!("{rs}");
}
