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
        n: usize,
        aabb: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    // https://atcoder.jp/contests/abc428/editorial/14240
    let calc_dist = |root: usize| -> Vec<isize> {
        let mut dist = vec![-1; n];
        let mut qq = VecDeque::new();
        dist[root] = 0;
        qq.push_front(root);
        while let Some(v) = qq.pop_front() {
            for d in g[v].iter().copied() {
                if dist[d] == -1 {
                    dist[d] = dist[v] + 1;
                    qq.push_back(d);
                }
            }
        }
        dist
    };
    let d0 = calc_dist(0);
    let u = d0.iter().position_max().unwrap();
    let du = calc_dist(u);
    let v = du.iter().position_max().unwrap();
    let dv = calc_dist(v);
    for i in 0..n {
        let rs = match du[i].cmp(&dv[i]) {
            Ordering::Greater => u + 1,
            Ordering::Equal => u.max(v) + 1,
            Ordering::Less => v + 1,
        };
        println!("{rs}");
    }
}
