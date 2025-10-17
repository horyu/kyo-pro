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
        t: usize,
        nn: [(usize, usize, usize); t],
    };
    for (mut na, nb, mut nc) in nn {
        let min = na.min(nb).min(nc);
        let mut rs = min;
        na -= min;
        nc -= min;
        if 0 < na && 0 < nc {
            // aac のパターンがx、accのパターンがyとして x+y=k を最大化
            // 2x+y <= na, x+2y <= nc
            // 3x+3y <= na+nc
            rs += na.min(nb).min((na + nc) / 3);
        }
        println!("{rs}");
    }
}
