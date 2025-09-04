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
        ss: [Chars; h],
    };
    let hh = h - 1;
    let ww = w - 1;
    let mut rs = vec![vec!['.'; w]; h];
    let mut tt = rs.clone();
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.iter().copied().enumerate() {
            if c == '#' && mawari(i, j, hh, ww).all(|(x, y)| ss[x][y] == '#') {
                rs[i][j] = '#';
            }
        }
    }
    for (i, s) in rs.iter().enumerate() {
        for (j, c) in s.iter().copied().enumerate() {
            if c == '#' {
                tt[i][j] = '#';
                for (x, y) in mawari(i, j, hh, ww) {
                    tt[x][y] = '#';
                }
            }
        }
    }
    if ss == tt {
        println!("possible");
        for rs in rs {
            println!("{}", rs.iter().join(""));
        }
    } else {
        println!("impossible");
    }
}

fn mawari(i: usize, j: usize, hh: usize, ww: usize) -> impl Iterator<Item = (usize, usize)> {
    let tate = match i {
        0 if hh == 0 => vec![0],
        0 => vec![0, 1],
        i if i == hh => vec![i - 1, i],
        i => vec![i - 1, i, i + 1],
    };
    let yoko = match j {
        0 if ww == 0 => vec![0],
        0 => vec![0, 1],
        j if j == ww => vec![j - 1, j],
        j => vec![j - 1, j, j + 1],
    };
    tate.into_iter()
        .cartesian_product(yoko.into_iter())
        .filter(move |&(ii, jj)| ii != i || jj != j)
}
