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
        n: usize,
        aabb: [(Usize1, Usize1); n - 1],
        q: usize,
        tteexx: [(usize, Usize1, isize); q],
    };
    // 頂点0を根として ab のどちらが根側か区別する
    // a側なら sum[0] += c, sum[b] -= c
    // sum を使ってDFS
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dd = vec![0; n];
    let mut qq = VecDeque::new();
    qq.push_back((0, 0, 0));
    while let Some((qi, qd, qf)) = qq.pop_front() {
        dd[qi] = qd;
        for i in g[qi].iter().copied() {
            if i != qf {
                qq.push_back((i, qd + 1, qi));
            }
        }
    }
    let mut ss: Vec<isize> = vec![0isize; n];
    for (t, e, x) in tteexx {
        let (i, j) = if t == 1 {
            aabb[e]
        } else {
            (aabb[e].1, aabb[e].0)
        };
        if dd[i] < dd[j] {
            ss[0] += x;
            ss[j] -= x;
        } else {
            ss[i] += x;
        }
    }
    let mut rs = vec![0; n];
    let mut qq = VecDeque::new();
    qq.push_back((0, 0, 0));
    while let Some((qi, qs, qf)) = qq.pop_front() {
        let s = qs + ss[qi];
        rs[qi] = s;
        for i in g[qi].iter().copied() {
            if i != qf {
                qq.push_back((i, s, qi));
            }
        }
    }
    let rs = rs.iter().join("\n");
    println!("{rs}");
}
