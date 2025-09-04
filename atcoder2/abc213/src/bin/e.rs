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
    // https://blog.hamayanhamayan.com/entry/2021/08/09/010308
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
        vv.retain(|&(vi, vj)| ss[vi][vj] != '#');
        vv
    };

    let mut d = vec![vec![1usize << 60; w]; h];
    d[0][0] = 0;
    let mut qq = VecDeque::new();
    qq.push_back((0, 0));
    let mut viewed = vec![vec![false; w]; h];
    while let Some((qx, qy)) = qq.pop_front() {
        if viewed[qx][qy] {
            continue;
        }
        viewed[qx][qy] = true;
        for (i, j) in udlr(qx, qy) {
            if d[qx][qy] < d[i][j] {
                d[i][j] = d[qx][qy];
                qq.push_front((i, j));
            }
        }
        for i in (qx.saturating_sub(2))..(h.min(qx + 3)) {
            for j in (qy.saturating_sub(2))..(w.min(qy + 3)) {
                if 4 <= i.abs_diff(qx) + j.abs_diff(qy) {
                    continue;
                }
                if d[qx][qy] + 1 < d[i][j] {
                    d[i][j] = d[qx][qy] + 1;
                    qq.push_back((i, j));
                }
            }
        }
    }
    let rs = d[h - 1][w - 1];
    println!("{rs}");
}
