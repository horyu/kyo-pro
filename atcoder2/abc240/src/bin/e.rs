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
        uuvv: [(Usize1, Usize1); n - 1],
    };
    // https://atcoder.jp/contests/abc240/editorial/3426
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // vv[葉] = 何番目の葉か
    let mut vv = vec![!0; n];
    dfs1(&g, &mut vv, 0, !0, &mut 0);

    let mut ll = vec![!0; n];
    let mut rr = vec![0; n];
    dfs2(&g, &vv, &mut ll, &mut rr, 0, !0);

    for (l, r) in izip!(ll, rr) {
        println!("{} {}", l + 1, r + 1);
    }
}

fn dfs1(g: &[Vec<usize>], leaves: &mut Vec<usize>, i: usize, p: usize, num: &mut usize) {
    if i != 0 && g[i].len() == 1 {
        leaves[i] = *num;
        *num += 1;
        return;
    }
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs1(g, leaves, j, i, num);
    }
}

fn dfs2(
    g: &[Vec<usize>],
    vv: &[usize],
    ll: &mut Vec<usize>,
    rr: &mut Vec<usize>,
    i: usize,
    p: usize,
) {
    if vv[i] != !0 {
        ll[i] = vv[i];
        rr[i] = vv[i];
        return;
    }
    for j in g[i].iter().copied() {
        if j == p {
            continue;
        }
        dfs2(g, vv, ll, rr, j, i);
        ll[i] = ll[i].min(ll[j]);
        rr[i] = rr[i].max(rr[j]);
    }
}
