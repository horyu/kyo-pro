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
        t: usize,
        mut aabb: [(usize, usize); n],
    };
    aabb.sort_unstable();

    let mut hm = HashMap::new();
    hm.insert(0usize, 0usize);

    let mut rs = 0;
    for (a, b) in aabb {
        let mut new_hm = hm.clone();
        for (k, v) in hm {
            let kk = k + a;
            let vv = v + b;
            rs = rs.max(vv);

            if kk < t {
                let e = new_hm.entry(kk).or_insert(0);
                *e = (*e).max(vv);
            }
        }
        hm = new_hm;
    }
    println!("{rs}");
}
