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
        aabb: [(Usize1, Usize1); n - 1],
        cc: [usize; n],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    // https://atcoder.jp/contests/abc348/editorial/9706
    // sum_c[i]: 頂点 i の部分木の頂点 x について C[x] の総和
    // sum_d[i]: 頂点 i の部分木の頂点 x について C[x] x d(i, x) の総和
    fn dfs(
        g: &[Vec<usize>],
        cc: &[usize],
        sum_c: &mut [usize],
        sum_d: &mut [usize],
        crr: usize,
        from: usize,
    ) {
        sum_c[crr] = cc[crr];
        sum_d[crr] = 0;
        for &to in &g[crr] {
            if to == from {
                continue;
            }
            dfs(g, cc, sum_c, sum_d, to, crr);
            sum_c[crr] += sum_c[to];
            sum_d[crr] += sum_c[to] + sum_d[to];
        }
    }
    let mut sum_c = vec![0; n];
    let mut sum_d = vec![0; n];
    dfs(&g, &cc, &mut sum_c, &mut sum_d, 0, !0);

    // ff[i]: f(i) の値
    // p_sum_c: v の部分木以外の頂点 x について C[x] の総和
    // p_sum_d: v の部分木以外の頂点 x について C[x] x d(v, x) の総和
    #[allow(clippy::too_many_arguments)]
    fn rerooting(
        g: &[Vec<usize>],
        sum_c: &[usize],
        sum_d: &[usize],
        ff: &mut [usize],
        crr: usize,
        from: usize,
        p_sum_c: usize,
        p_sum_d: usize,
    ) {
        ff[crr] = sum_d[crr] + p_sum_d;
        for &to in &g[crr] {
            if to == from {
                continue;
            }
            let nc = p_sum_c + sum_c[crr] - sum_c[to];
            let nd = p_sum_d + sum_d[crr] - sum_d[to] - sum_c[to] + nc;
            rerooting(g, sum_c, sum_d, ff, to, crr, nc, nd);
        }
    }
    let mut ff = vec![0; n];
    rerooting(&g, &sum_c, &sum_d, &mut ff, 0, !0, 0, 0);

    let rs = ff.into_iter().min().unwrap();
    println!("{rs}");
}
