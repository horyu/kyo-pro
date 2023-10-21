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
        n: usize,
        q: usize,
        xx: [usize; n],
        aabb: [(Usize1, Usize1); n - 1],
        vvkk: [(Usize1, Usize1); q],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut i2vv = (0..n).map(|i| vec![xx[i]]).collect_vec();
    dfs(&mut i2vv, 0, !0, &g);
    for (v, k) in vvkk {
        let rs = i2vv[v][k];
        println!("{rs}");
    }
}

fn dfs(i2vv: &mut Vec<Vec<usize>>, crr: usize, from: usize, g: &[Vec<usize>]) {
    let mut vv = std::mem::take(&mut i2vv[crr]);
    for &i in &g[crr] {
        if i != from {
            dfs(i2vv, i, crr, g);
            vv.extend(i2vv[i].iter().take(20));
        }
    }
    vv.sort_unstable();
    vv.reverse();
    vv.truncate(20);
    i2vv[crr] = vv;
}
