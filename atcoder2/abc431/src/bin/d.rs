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
    let w_sum = wwhhbb.iter().map(|(w, _, _)| *w).sum::<usize>();
    let mut dp0 = vec![0];
    let mut dp1 = vec![];
    for (w, h, b) in wwhhbb {
    }
    // let rs = btm
    //     .into_iter()
    //     .filter_map(|(wh, score)| (wh <= w_sum - wh).then_some(score))
    //     .max()
    //     .unwrap_or_default();
    // println!("{rs}");
}
