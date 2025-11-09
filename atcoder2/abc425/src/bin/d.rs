#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools, chain, iproduct, izip};
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
        h: usize,
        w: usize,
        ss: [Chars; h],
    };
    let udlr = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut vv = vec![];
        if 0 < i {
            vv.push((i - 1, j));
        }
        if i < h - 1 {
            vv.push((i + 1, j));
        }
        if 0 < j {
            vv.push((i, j - 1));
        }
        if j < w - 1 {
            vv.push((i, j + 1));
        }
        vv
    };
    let mut xxx = ss
        .into_iter()
        .map(|s| s.into_iter().map(|c| c == '#').collect_vec())
        .collect_vec();
    let mut vv = xxx
        .iter()
        .enumerate()
        .flat_map(|(i, xx)| xx.iter().positions(|&x| x).map(move |j| (i, j)))
        .collect_vec();
    let mut checked = vec![vec![false; w]; h];
    for (i, j) in vv.iter().copied() {
        checked[i][j] = true;
    }
    while !vv.is_empty() {
        let mut ww = vec![];
        for (i, j) in vv {
            for (ni, nj) in udlr(i, j) {
                if checked[ni][nj] {
                    continue;
                }
                checked[ni][nj] = true;
                if udlr(ni, nj)
                    .into_iter()
                    .filter(|&(nni, nnj)| xxx[nni][nnj])
                    .exactly_one()
                    .is_ok()
                {
                    ww.push((ni, nj));
                }
            }
        }
        for (i, j) in ww.iter().copied() {
            xxx[i][j] = true;
        }
        vv = ww;
    }
    let rs = xxx.into_iter().flatten().map(usize::from).sum::<usize>();
    println!("{rs}");
}
