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
        h: usize,
        m: usize,
        aabb: [(usize, usize); n],
    };
    let mut hs = HashSet::new();
    hs.insert((0, 0));
    for (qi, (a, b)) in aabb.into_iter().enumerate() {
        // let mut xx2 = BTreeMap::new();
        // let mut yy2 = BTreeMap::new();
        let mut new_hs = HashSet::new();
        for (x, y) in hs {
            let (xx, yy) = (x + a, y + b);
            if xx <= h {
                new_hs.insert((xx, y));

            }
            if yy <= m {
                new_hs.insert((x, yy));
            }
        }
        if new_hs.is_empty() {
            println!("{qi}");
            return;
        }
        hs = new_hs;
    }
    println!("{n}");
}
