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
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // 頂点ごとの次数
    let degree = g.iter().map(|ii| ii.len()).collect_vec();
    // 次数が3のグループと隣接した次数が2の頂点対を見つける
    let mut dsu = ac_library::Dsu::new(n);
    for (u, v) in uuvv.iter().copied() {
        if degree[u] == 3 && degree[v] == 3 {
            dsu.merge(u, v);
        }
    }
    let mut counter = counter::Counter::<_>::new();
    for (u, v) in uuvv.iter().copied() {
        if degree[u] == 3 && degree[v] == 2 {
            counter[&dsu.leader(u)] += 1;
        } else if degree[u] == 2 && degree[v] == 3 {
            counter[&dsu.leader(v)] += 1;
        }
    }
    let rs = counter
        .into_iter()
        .fold(0, |acc, (_k, cnt)| acc + cnt * (cnt - 1) / 2);
    println!("{rs}");
}
