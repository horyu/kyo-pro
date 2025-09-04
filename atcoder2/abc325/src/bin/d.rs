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
        ttdd: [(usize, usize); n],
    };
    let mut mm = btreemultimap::BTreeMultiMap::new();
    for (t, d) in ttdd.iter().copied() {
        mm.insert(t, R(t + d));
    }
    let mut rs = 0usize;
    let mut now = 1;
    let mut bh = BinaryHeap::new();
    loop {
        if let Some(vv) = mm.remove(&now) {
            bh.extend(vv);
        }
        while let Some(R(v)) = bh.pop() {
            if v < now {
                continue;
            }
            rs += 1;
            now += 1;
            break;
        }
        if bh.is_empty() {
            if let Some((&next, _)) = mm.range(now..).next() {
                now = next;
            } else {
                break;
            }
        }
    }
    println!("{rs}");
}
