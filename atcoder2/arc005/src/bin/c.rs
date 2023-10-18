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
        h: usize,
        w: usize,
        ccc: [Chars; h],
    };

    let mut s = (0, 0);
    for (i, cc) in ccc.iter().enumerate() {
        for (j, c) in cc.iter().copied().enumerate() {
            if c == 's' {
                s = (i, j)
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
        vv
    };

    let mut qq = VecDeque::new();
    let mut dist = vec![vec![std::usize::MAX; w]; h];
    qq.push_front(s);
    dist[s.0][s.1] = 0;
    while let Some((qi, qj)) = qq.pop_front() {
        let qd = dist[qi][qj];
        for (i, j) in udlr(qi, qj) {
            if ccc[i][j] == 'g' {
                println!("YES");
                return;
            }
            let wall = usize::from(ccc[i][j] == '#');
            let d = qd + wall;
            if d < dist[i][j].min(3) {
                dist[i][j] = d;
                if wall == 0 {
                    qq.push_front((i, j));
                } else {
                    qq.push_back((i, j));
                }
            }
        }
    }
    println!("NO");
}
