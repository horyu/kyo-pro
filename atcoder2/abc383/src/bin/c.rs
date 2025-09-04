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
    let mut hs = HashSet::new();
    let mut qq = VecDeque::new();
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.iter().copied().enumerate() {
            if c == 'H' {
                hs.insert((i, j));
                qq.push_back((i, j, 0));
            }
        }
    }
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
        vv.retain(|&(vi, vj)| ss[vi][vj] == '.');
        vv
    };
    while let Some((qi, qj, qd)) = qq.pop_front() {
        if qd < d {
            for (i, j) in udlr(qi, qj) {
                if hs.insert((i, j)) {
                    qq.push_back((i, j, qd + 1));
                }
            }
        }
    }
    let rs = hs.len();
    println!("{rs}");
}
