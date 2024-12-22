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
        uuvvww: [(Usize1, Usize1, isize); m],
    };
    let mut g = vec![vec![]; n];
    let mut dsu = ac_library::Dsu::new(n);
    for &(u, v, w) in &uuvvww {
        g[u].push((v, w));
        g[v].push((u, -w));
        dsu.merge(u, v);
    }
    let mut rrss = vec![0isize; n];
    let mut pushed = vec![false; n];
    for group in dsu.groups() {
        let mut qq = VecDeque::new();
        qq.push_back(group[0]);
        pushed[group[0]] = true;
        while let Some(u) = qq.pop_front() {
            for &(v, w) in &g[u] {
                if pushed[v] {
                    continue;
                }
                pushed[v] = true;
                qq.push_back(v);
                rrss[v] = rrss[u] + w;
            }
        }
    }
    // for (u, v, w) in uuvvww {
    //     assert!(rrss[v] - rrss[u] == w);
    // }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
