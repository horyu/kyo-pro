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
        m: usize,
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    let mut used = vec![false; n];
    let mut rs = 0;
    dfs(&g, &mut used, &mut rs, 0);
    println!("{rs}");
}

fn dfs(g: &[Vec<usize>], used: &mut [bool], cnt: &mut usize, crr: usize) {
    if 1e6 as usize <= *cnt {
        return;
    }
    used[crr] = true;
    *cnt += 1;
    for &v in &g[crr] {
        if used[v] {
            continue;
        }
        dfs(g, used, cnt, v);
    }
    used[crr] = false;
}
