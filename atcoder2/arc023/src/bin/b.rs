#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        r: usize,
        c: usize,
        d: usize,
        aaa: [[usize; c]; r],
    };
    let mut rs = 0;
    for i in 0..r {
        if d < i {
            break;
        }
        for j in 0..c {
            if d < i + j {
                break;
            }
            if (i + j) % 2 == d % 2 {
                rs = rs.max(aaa[i][j]);
            }
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        r: usize,
        c: usize,
        d: usize,
        aaa: [[usize; c]; r],
    };
    let udlr = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut vv = vec![];
        if 0 < i {
            vv.push((i - 1, j));
        }
        if i < r - 1 {
            vv.push((i + 1, j));
        }
        if 0 < j {
            vv.push((i, j - 1));
        }
        if j < c - 1 {
            vv.push((i, j + 1));
        }
        vv
    };
    let mut hhss = [HashSet::new(), HashSet::new()];
    hhss[0].insert((0, 0));
    let mut tmp = HashSet::from([(0, 0)]);
    for dd in 0..d {
        let mut new_tmp = HashSet::new();
        for (i, j) in tmp {
            for (ni, nj) in udlr(i, j) {
                if hhss[(dd + 1) % 2].insert((ni, nj)) {
                    new_tmp.insert((ni, nj));
                }
            }
        }
        if new_tmp.is_empty() {
            break;
        }
        tmp = new_tmp;
    }
    let rs = hhss[d % 2]
        .iter()
        .copied()
        .map(|(i, j)| aaa[i][j])
        .max()
        .unwrap_or_default();
    println!("{rs}");
}
