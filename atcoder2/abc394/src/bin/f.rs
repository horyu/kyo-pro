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
        aabb: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dsu = ac_library::Dsu::new(n);
    for (i, vv) in g.iter().enumerate() {
        if 4 <= vv.len() {
            for v in vv.iter().copied() {
                dsu.merge(i, v);
            }
        }
    }
    let mut rs = 2.min(n);
    // let mut memo = HashMap::new();
    for vv in dsu.groups() {
        if vv.len() == 1 {
            continue;
        }
        // TODO
    }
    println!("{rs}");
}
