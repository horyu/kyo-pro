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
    };
    let mut hm = HashMap::new();
    let mut g = vec![];
    for _ in 0..n {
        input! {m: usize, ppee: [(usize, usize); m]};
        for (p, e) in ppee.iter().copied() {
            hm.entry(p).or_insert_with(Vec::new).push(e);
        }
        g.push(ppee);
    }
    for vv in hm.values_mut() {
        vv.sort_unstable();
        vv.reverse();
    }
    let mut rs = 1usize;
    for ppee in g {
        // eprintln!("{:?}", &ppee);
        if ppee.into_iter().any(|(p, e)| {
            let vv = hm.get(&p).unwrap();
            vv[0] == e && vv.get(1).copied().unwrap_or_default() < e
        }) {
            // dbg!(1);
            rs += 1;
        }
    }
    rs = rs.min(n);
    println!("{rs}");
}
