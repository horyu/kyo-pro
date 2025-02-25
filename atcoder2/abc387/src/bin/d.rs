#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
    let (mut si, mut sj) = (0, 0);
    let (mut gi, mut gj) = (0, 0);
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.iter().copied().enumerate() {
            match c {
                'S' => {
                    (si, sj) = (i, j);
                }
                'G' => {
                    (gi, gj) = (i, j);
                }
                _ => (),
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
        vv.retain(|&(vi, vj)| ss[vi][vj] != '#');
        vv
    };
    let mut qq = VecDeque::new();
    let mut pushed = vec![vec![[false; 2]; w]; h];
    for dir in 0..2 {
        qq.push_back((si, sj, 0, dir));
        pushed[si][sj][dir] = true;
    }
    while let Some((qi, qj, qd, qdir)) = qq.pop_front() {
        if (qi, qj) == (gi, gj) {
            println!("{qd}");
            return;
        }
        let qa = ss[qi][qj];
        for (i, j) in udlr(qi, qj) {
            let dir = usize::from(qi == i);
            if pushed[i][j][dir] || qdir == dir {
                continue;
            }
            pushed[i][j][dir] = true;
            qq.push_back((i, j, qd + 1, dir));
        }
    }
    println!("-1");
}
