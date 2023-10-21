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
        s: Bytes,
        t: Bytes,
    };
    let s = s.into_iter().map(|b| b as usize).collect_vec();
    let t = t.into_iter().map(|b| b as usize).collect_vec();

    let mut bbttss = vec![BTreeSet::new(); 256];
    for (i, b) in s.iter().copied().enumerate() {
        bbttss[b].insert(i);
    }
    if t.iter().copied().any(|b| bbttss[b].is_empty()) {
        println!("-1");
        return;
    }
    let mut cycle = 0;
    let mut pos = 0;
    for b in t.iter().copied() {
        let bts = &bbttss[b];
        if let Some(i) = bts.range(pos..).next().copied() {
            pos = i + 1;
            continue;
        }
        if let Some(i) = bts.iter().min().copied() {
            pos = i + 1;
            cycle += 1;
        }
    }
    let rs = cycle * s.len() + pos;
    println!("{rs}");
}
