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
        h: usize,
        w: usize,
        d: usize,
        ss: [Chars; h],
    };
    let iter = ss.iter().enumerate().flat_map(|(i, s)| {
        s.iter()
            .enumerate()
            .filter_map(move |(j, &c)| (c == '.').then_some((i, j)))
    });
    let rs = iter
        .clone()
        .map(|(i, j)| {
            HashSet::<(_, _)>::from_iter(
                iter.clone()
                    .filter(|&(ii, jj)| i.abs_diff(ii) + j.abs_diff(jj) <= d),
            )
        })
        .tuple_combinations()
        .map(|(a, b)| a.union(&b).count())
        .max()
        .unwrap_or_default();
    println!("{rs}");
}
