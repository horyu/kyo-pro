#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        wwvv: [(usize, usize); n],
    };
    let mut hm = HashMap::new();
    hm.insert(0, 0);
    for (w, v) in wwvv {
        let mut new_hm = hm.clone();
        for (a, b) in hm {
            if a + w <= k {
                let e = new_hm.entry(a + w).or_insert(0);
                *e = (*e).max(b + v);
            }
        }
        hm = new_hm;
    }
    let rs = hm.into_values().max().unwrap();
    println!("{rs}");
}