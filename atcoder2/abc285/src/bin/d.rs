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
        sstt: [(String, String); n],
    };
    let mut hs = HashSet::new();
    for (s, t) in &sstt {
        hs.insert(s);
        hs.insert(t);
    }
    let s2i = hs
        .iter()
        .copied()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();
    let mut uf = UnionFind::new(s2i.len());
    for (s, t) in &sstt {
        if !uf.union(s2i[s], s2i[t]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
