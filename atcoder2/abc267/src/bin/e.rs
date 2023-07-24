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
        aa: [usize; n],
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    let mut cc = vec![0; n];
    for (u, v) in uuvv {
        cc[u] += aa[v];
        cc[v] += aa[u];
        g[u].push(v);
        g[v].push(u);
    }
    let mut bh = BinaryHeap::new();
    for i in 0..n {
        bh.push((std::cmp::Reverse(cc[i]), i));
    }
    let mut rs = 0;
    let mut removed = vec![false; n];
    while let Some((std::cmp::Reverse(c), i)) = bh.pop() {
        if removed[i] {
            continue;
        }
        removed[i] = true;
        rs = rs.max(c);
        for &j in &g[i] {
            if removed[j] {
                continue;
            }
            cc[j] -= aa[i];
            bh.push((std::cmp::Reverse(cc[j]), j));
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![HashSet::new(); n];
    let mut cc = vec![0usize; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].insert(v);
        g[v].insert(u);
        cc[u] += aa[v];
        cc[v] += aa[u];
    }
    let mut btm = BTreeMap::new();
    for (i, c) in cc.iter().copied().enumerate() {
        btm.entry(c).or_insert_with(HashSet::new).insert(i);
    }
    let mut rs = 0usize;
    while let Some(c) = btm.keys().next().copied() {
        rs = rs.max(c);
        if let Some(removed) = btm.remove(&c) {
            for i in removed {
                let a = aa[i];
                let jj = std::mem::take(&mut g[i]);
                for j in jj {
                    if let Some(hs) = btm.get_mut(&cc[j]) {
                        hs.remove(&j);
                        if hs.is_empty() {
                            btm.remove(&cc[j]);
                        }
                        cc[j] -= a;
                        btm.entry(cc[j]).or_insert_with(HashSet::new).insert(j);
                    }
                    g[j].remove(&i);
                }
            }
        }
    }
    println!("{rs}");
}
