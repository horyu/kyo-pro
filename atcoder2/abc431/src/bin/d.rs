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
        wwhhbb: [(usize, usize, usize); n],
    };
    // https://atcoder.jp/contests/abc431/editorial/14536
    let mut dp = vec![0];
    let mut prev = vec![];
    for (w, h, b) in wwhhbb {
        std::mem::swap(&mut dp, &mut prev);
        let m = prev.len();
        dp.resize(m + w, 0);
        for i in 0..m {
            dp[i] = dp[i].max(prev[i] + h);
            dp[i + w] = dp[i + w].max(prev[i] + b);
        }
    }
    let rs = dp[(dp.len() / 2)..].iter().max().unwrap();
    println!("{rs}");
}
