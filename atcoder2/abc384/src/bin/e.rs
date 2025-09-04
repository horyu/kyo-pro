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
        x: u128,
        p: Usize1,
        q: Usize1,
        sss: [[u128; w]; h],
    };
    let mut bh = BinaryHeap::new();
    let mut hs = HashSet::new();
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
    for (i, j) in udlr(p, q) {
        bh.push((R(sss[i][j]), i, j));
        hs.insert((i, j));
    }
    hs.insert((p, q));
    let mut rs = sss[p][q];
    while let Some((R(qp), qi, qj)) = bh.pop() {
        if qp * x < rs {
            rs += qp;
            for (i, j) in udlr(qi, qj) {
                if hs.insert((i, j)) {
                    bh.push((R(sss[i][j]), i, j));
                }
            }
        }
    }
    println!("{rs}");
}
