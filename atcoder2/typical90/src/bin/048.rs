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
        k: usize,
        aabb: [(usize, usize); n],
    };
    let mut bh: BinaryHeap<_> = aabb
        .iter()
        .copied()
        .enumerate()
        .map(|(i, ab)| (ab.1, i, true))
        .collect();
    let mut rs = 0;
    for _ in 0..k {
        let (score, i, is_bubun) = bh.pop().unwrap();
        rs += score;
        if is_bubun {
            bh.push((aabb[i].0 - score, i, false));
        }
    }
    println!("{rs}");
}
