#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use pathfinding::prelude::dijkstra;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        uuvvww: [(Usize1, Usize1, usize); m],
    };
    let mut g = vec![btreemultimap::BTreeMultiMap::new(); n];
    let mut rs = 0;
    for (u, v, w) in uuvvww {
        g[u].insert(w, v);
        g[v].insert(w, u);
        rs |= w;
    }
    for d in (0..30).rev() {
        if rs & (1 << d) == 0 {
            continue;
        }
        let tmp = rs ^ (1 << d);
        if pathfinding::prelude::dfs(
            0,
            |&i| {
                g[i].range(..=tmp)
                    .filter_map(|(&k, &v)| (tmp & k == k).then_some(v))
            },
            |&i| i == n - 1,
        )
        .is_some()
        {
            rs = tmp;
        }
    }
    println!("{rs}");
}
