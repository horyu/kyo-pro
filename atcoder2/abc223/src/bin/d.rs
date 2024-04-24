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
        aabb: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    let mut pp = vec![0; n];
    let mut dsu = ac_library::Dsu::new(n);
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        pp[b] += 1;
        dsu.merge(a, b);
    }
    let mut bh = BinaryHeap::new();
    for (i, p) in pp.iter().copied().enumerate() {
        if p == 0 {
            bh.push(R(i));
        }
    }
    let mut rrss = vec![];
    while let Some(R(i)) = bh.pop() {
        rrss.push(i + 1);
        for &j in &g[i] {
            pp[j] -= 1;
            if pp[j] == 0 {
                bh.push(R(j));
            }
        }
    }
    if rrss.len() < n {
        println!("-1");
        return;
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
