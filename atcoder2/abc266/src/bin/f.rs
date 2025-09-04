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
        n: usize,
        uuvv: [(Usize1, Usize1); n],
        q: usize,
        xxyy: [(Usize1, Usize1); q],
    };
    let mut g = vec![vec![]; n];
    let mut cc = vec![0; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
        cc[u] += 1;
        cc[v] += 1;
    }
    let mut qq = VecDeque::new();
    for (i, c) in cc.iter().copied().enumerate() {
        if c == 1 {
            qq.push_back(i);
        }
    }
    let mut is_cycle = vec![true; n];
    while let Some(i) = qq.pop_front() {
        is_cycle[i] = false;
        for j in g[i].iter().copied() {
            if 2 <= cc[j] {
                cc[j] -= 1;
                if cc[j] == 1 {
                    qq.push_back(j);
                }
            }
        }
    }
    let mut root_id = vec![!0; n];
    for i in is_cycle.iter().copied().positions(|tf| tf) {
        root_id[i] = i;
        for j in g[i].iter().copied() {
            if !is_cycle[j] {
                dfs(&g, &mut root_id, j, i);
            }
        }
    }
    fn dfs(g: &[Vec<usize>], root_id: &mut Vec<usize>, v: usize, p: usize) {
        root_id[v] = root_id[p];
        for to in g[v].iter().copied() {
            if p != to {
                dfs(g, root_id, to, v);
            }
        }
    }
    for (x, y) in xxyy {
        let rs = ["No", "Yes"][(root_id[x] == root_id[y]) as usize];
        println!("{rs}");
    }
}
