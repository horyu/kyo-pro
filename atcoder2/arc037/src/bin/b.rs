#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
    let mut dsu = ac_library::Dsu::new(n);
    for (u, v) in uuvv.iter().copied() {
        dsu.merge(u, v);
    }
    let mut mm = btreemultimap::BTreeMultiMap::new();
    for (i, (u, _)) in uuvv.iter().copied().enumerate() {
        mm.insert(dsu.leader(u), i);
    }
    let mut uf = UnionFind::new(n);
    let mut rs = dsu.groups().len();
    for (_, ii) in mm {
        if ii.into_iter().any(|i| {
            let (u, v) = uuvv[i];
            !uf.union(u, v)
        }) {
            rs -= 1;
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
        uuvv: [(Usize1, Usize1); m],
    };
    let mut uf = UnionFind::new(n);
    let mut ng = HashSet::new();
    for (u, v) in uuvv {
        if uf.find(u) == uf.find(v) {
            ng.insert(u);
        }
        uf.union(u, v);
    }
    ng = ng.into_iter().map(|i| uf.find(i)).collect();
    let labels: HashSet<usize> = uf.into_labeling().into_iter().collect();
    let rs = labels
        .into_iter()
        .unique()
        .filter(|i| !ng.contains(i))
        .count();
    println!("{rs}");
}
