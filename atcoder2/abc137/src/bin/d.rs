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
        aabb: [(Usize1, usize); n],
    };
    let mut a2bb = vec![vec![]; 1e5 as usize];
    for (a, b) in aabb {
        a2bb[a].push(b);
    }
    let mut bh = BinaryHeap::new();

    let mut rs = 0usize;
    for aa in a2bb.into_iter().take(m) {
        bh.extend(aa);

        if let Some(b) = bh.pop() {
            rs += b;
        }
    }
    println!("{rs}");
}
