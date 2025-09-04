#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, join, Itertools as _};
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
    let mut sml = ac_library::Dsu::new(n);
    let mut big = ac_library::Dsu::new(n);
    for i in 0..n {
        for j in g[i].iter().copied() {
            if j < i {
                sml.merge(i, j);
            } else {
                big.merge(i, j);
            }
        }
        let rs = if sml.size(i) == i + 1 {
            (big.size(i) - (i + 1)) as i64
        } else {
            -1
        };
        println!("{rs}");
    }
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![BTreeSet::new(); n];
    for (u, v) in uuvv.iter().copied() {
        g[u].insert(v);
        g[v].insert(u);
    }
    // 0:                      g[0][1..]
    // 1: g[1][..1]が存在する ? g[0][2..] & g[1][2..] : -1
    // 2: g[2][..2]が存在する ? g[..=2][3..] : -1
    let mut dsu = ac_library::Dsu::new(n);
    let mut bts = BTreeSet::from_iter(1..n);
    // let mut bts = BTreeSet::new();
    for k in 0..n {
        for i in g[k].range(..k).copied() {
            dsu.merge(k, i);
        }
        if dsu.size(0) == k + 1 {
            // 全てg[0]にまとめる
            while let Some(&i) = bts.range(..=k).next() {
                bts.remove(&i);
                let (sml, big) = if g[0].len() < g[i].len() {
                    (0, i)
                } else {
                    (i, 0)
                };
                let tmp = std::mem::take(&mut g[sml]);
                g[big].extend(tmp);
                g.swap(0, big);
            }
            // g[0][..=k] を削除
            while let Some(&i) = g[0].range(..=k).next() {
                g[0].remove(&i);
            }
            let rs = g[0].len();
            println!("{rs}");
        } else {
            println!("-1");
        }
    }
}
