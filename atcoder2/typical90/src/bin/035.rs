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
    };
    // https://github.com/E869120/kyopro_educational_90/blob/main/sol/035-04.cpp
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    const BITS: usize = 100000usize.ilog2() as usize + 1;
    let mut par = vec![vec![0; n]; BITS];
    let mut depth = vec![0; n];
    let mut id = vec![0; n];
    let mut vert_id = 0;
    build_tree(0, 0, &mut par, &mut id, &mut depth, &mut vert_id, &g);
    for i in 1..BITS {
        for j in 0..n {
            par[i][j] = par[i - 1][par[i - 1][j]];
        }
    }
    let lca = |va: usize, vb: usize| -> usize {
        let (mut va, mut vb) = if depth[va] < depth[vb] {
            (vb, va)
        } else {
            (va, vb)
        };
        for i in (0..BITS).rev() {
            if (1 << i) <= depth[va] - depth[vb] {
                va = par[i][va];
            }
        }
        if va == vb {
            return va;
        }
        for i in (0..BITS).rev() {
            if par[i][va] != par[i][vb] {
                va = par[i][va];
                vb = par[i][vb];
            }
        }
        par[0][va]
    };
    for _ in 0..q {
        input! {
            k: usize,
            mut vv: [Usize1; k],
        };
        vv.sort_unstable_by_key(|&v| id[v]);
        let mut rs = 0;
        for i in 0..k {
            rs += depth[vv[i]];
            rs -= depth[lca(vv[i], vv[(i + 1) % k])];
        }
        println!("{rs}");
    }
}

fn build_tree(
    pos: usize,
    pre: usize,
    par: &mut Vec<Vec<usize>>,
    id: &mut Vec<usize>,
    depth: &mut Vec<isize>,
    vert_id: &mut usize,
    g: &Vec<Vec<usize>>,
) {
    par[0][pos] = pre;
    id[pos] = *vert_id;
    *vert_id += 1;
    for &i in g[pos].iter() {
        if i == pre {
            continue;
        }
        depth[i] = depth[pos] + 1;
        build_tree(i, pos, par, id, depth, vert_id, g);
    }
}
