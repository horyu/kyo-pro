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
        n: usize,
        xxcc: [(isize, Usize1); n],
    };
    let mut btm = BTreeMap::new();
    for (x, c) in xxcc.iter().copied() {
        btm.entry(c).or_insert_with(Vec::new).push(x);
    }
    let mut ppss = vec![(0isize, 0usize)];
    for xx in btm.into_values() {
        let mut new_ppss = vec![];
        match xx.into_iter().minmax() {
            itertools::MinMaxResult::NoElements => unreachable!(),
            itertools::MinMaxResult::OneElement(x) => {
                for (p, s) in ppss {
                    new_ppss.push((x, s + p.abs_diff(x)));
                }
            }
            itertools::MinMaxResult::MinMax(l, r) => {
                for (p, s) in ppss {
                    for (x0, x1) in [(l, r), (r, l)] {
                        new_ppss.push((x1, s + p.abs_diff(x0) + x0.abs_diff(x1)));
                    }
                }
            }
        }
        ppss = new_ppss;
        ppss.sort_unstable();
        ppss.dedup_by_key(|ps| ps.0);
    }
    let rs = ppss
        .into_iter()
        .map(|(p, s)| p.abs_diff(0) + s)
        .min()
        .unwrap();
    println!("{rs}");
}
