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
    let mut hm = HashMap::new();
    hm.insert((h, m), 0);
    for (qi, (a, b)) in aabb.into_iter().enumerate() {
        let mut new_hm = HashMap::new();
        for ((x, y), c) in hm {
            if a <= x {
                new_hm.insert((x - a, y), c + 1);
            }
            if b <= y {
                new_hm.insert((x, y - b), c + 1);
            }
        }
        if new_hm.is_empty() {
            println!("{qi}");
            return;
        }
        hm = new_hm;
    }
    println!("{n}");
}
