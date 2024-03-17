#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![feature(get_many_mut)]
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
        q: usize,
        cc: [Usize1; n],
        aabb: [(Usize1, Usize1); q],
    };
    let mut hhss = cc
        .into_iter()
        .map(|c| HashSet::<usize>::from_iter([c]))
        .collect_vec();
    for (a, b) in aabb {
        if let Ok([ahs, bhs]) = hhss.get_many_mut([a, b]) {
            if ahs.len() < bhs.len() {
                bhs.extend(ahs.drain());
            } else {
                ahs.extend(bhs.drain());
            }
            if bhs.is_empty() {
                std::mem::swap(ahs, bhs);
            }
            let rs = bhs.len();
            println!("{rs}");
        }
    }
}
