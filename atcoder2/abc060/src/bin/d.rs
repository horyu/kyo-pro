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
        w: usize,
        wwvv: [(usize, usize); n],
    };
    let mut hm = HashMap::new();
    hm.insert(0, 0);
    for (wi, vi) in wwvv {
        let mut new_hm = hm.clone();
        for (a, b) in hm {
            let ww = a + wi;
            if ww <= w {
                let e = new_hm.entry(ww).or_insert(0);
                *e = (*e).max(b + vi);
            }
        }
        hm = new_hm;
    }
    let rs = hm.into_values().max().unwrap();
    println!("{rs}");
}
