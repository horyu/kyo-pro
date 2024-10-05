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
        k: usize,
        aabb: [(Usize1, Usize1); m],
        ccdd: [(Usize1, Usize1); k],
    };
    let mut dsu = ac_library::Dsu::new(n);
    // 隣接数
    let mut adjs = vec![0; n];
    for (a, b) in aabb.iter().copied() {
        dsu.merge(a, b);
        adjs[a] += 1;
        adjs[b] += 1;
    }
    let mut ngs = vec![0; n];
    for (c, d) in ccdd.iter().copied() {
        if dsu.same(c, d) {
            ngs[c] += 1;
            ngs[d] += 1;
        }
    }
    let mut rrss = vec![0; n];
    for (i, (adj, ng)) in izip!(adjs, ngs).enumerate() {
        rrss[i] = dsu.size(i) - 1 - adj - ng;
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        aabb: [(Usize1, Usize1); m],
        ccdd: [(Usize1, Usize1); k],
    };
    let mut dsu = ac_library::Dsu::new(n);
    for (a, b) in aabb.iter().copied() {
        dsu.merge(a, b);
    }
    let mut rs = (0..n).map(|i| dsu.size(i) - 1).collect_vec();
    for (a, b) in aabb.iter().copied() {
        rs[a] -= 1;
        rs[b] -= 1;
    }
    for (c, d) in ccdd.iter().copied() {
        if dsu.same(c, d) {
            rs[c] -= 1;
            rs[d] -= 1;
        }
    }
    let rs = rs.iter().join(" ");
    println!("{rs}");
}
