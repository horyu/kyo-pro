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
    let w_sum = wwhhbb.iter().map(|(w, _, _)| *w).sum::<usize>();
    // head_weight -> score
    let mut btm = BTreeMap::new();
    btm.insert(0, 0);
    for (w, h, b) in wwhhbb {
        let mut new_btm = BTreeMap::new();
        for (wh, score) in btm {
            // h
            let eh = new_btm.entry(wh + w).or_insert(0);
            *eh = (*eh).max(score + h);
            // b
            let eb = new_btm.entry(wh).or_insert(0);
            *eb = (*eb).max(score + b);
        }
        btm = new_btm;
    }
    let rs = btm
        .into_iter()
        .filter_map(|(wh, score)| (wh <= w_sum - wh).then_some(score))
        .max()
        .unwrap_or_default();
    println!("{rs}");
}
