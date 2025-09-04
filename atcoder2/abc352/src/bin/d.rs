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
        k: usize,
        i2p: [Usize1; n],
    };
    let mut p2i = vec![0; n];
    for (i, p) in i2p.iter().copied().enumerate() {
        p2i[p] = i;
    }
    let mut bts = BTreeSet::new();
    for kk in 0..k {
        bts.insert(p2i[kk]);
    }
    let mut rs = bts.iter().max().unwrap() - bts.iter().min().unwrap();
    for kk in k..n {
        // eprintln!("{kk}: {bts:?}");
        bts.remove(&p2i[kk - k]);
        bts.insert(p2i[kk]);
        rs = rs.min(bts.iter().max().unwrap() - bts.iter().min().unwrap());
    }
    // eprintln!("{bts:?}");
    println!("{rs}");
}
