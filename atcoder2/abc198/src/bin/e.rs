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
        cc: [Usize1; n],
        aabb: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut rs = vec![];
    dfs(&g, &cc, !0, 0, &mut vec![0; 1e5 as usize], &mut rs);
    rs.sort_unstable();
    let rs = rs.iter().join("\n");
    println!("{rs}");
}

fn dfs(
    g: &[Vec<usize>],
    cc: &[usize],
    from: usize,
    to: usize,
    cc_cnt: &mut [usize],
    rs: &mut Vec<usize>,
) {
    cc_cnt[cc[to]] += 1;
    if cc_cnt[cc[to]] == 1 {
        rs.push(to + 1);
    }
    for next in g[to].iter().copied() {
        if next == from {
            continue;
        }
        dfs(g, cc, to, next, cc_cnt, rs);
    }
    cc_cnt[cc[to]] -= 1;
}
