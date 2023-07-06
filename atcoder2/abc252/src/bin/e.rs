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
        m: usize,
        aabbcc: [(Usize1, Usize1, usize); m],
    };
    let mut g = vec![vec![]; n];
    for (i, (u, v, c)) in aabbcc.iter().copied().enumerate() {
        g[u].push(i);
        g[v].push(i);
    }
    const X: usize = 1e17 as usize;
    // 1と接続した辺だけBHに突っ込む→POP→接続したらそこまでの距離を足した辺をBHに突っ込む
    let mut pushed_edge = vec![false; m];
    let mut checked_node = vec![false; n];
    let mut bh = BinaryHeap::new();
    for i in g[0].iter().copied() {
        pushed_edge[i] = true;
        bh.push((X - aabbcc[i].2, i));
    }
    checked_node[0] = true;
    let mut uf = UnionFind::new(n);
    let mut rs = vec![];
    while let Some((qrc, qi)) = bh.pop() {
        let qc = X - qrc;
        let (qa, qb, _) = aabbcc[qi];
        if uf.union(qa, qb) {
            rs.push(qi + 1);
            for i in [qa, qb] {
                if checked_node[i] {
                    continue;
                }
                checked_node[i] = true;
                for j in g[i].iter().copied() {
                    if pushed_edge[j] {
                        continue;
                    }
                    pushed_edge[j] = true;
                    bh.push((X - qc - aabbcc[j].2, j));
                }
            }
        }
    }
    let rs = rs.iter().join(" ");
    println!("{rs}");
}
