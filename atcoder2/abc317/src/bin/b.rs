#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        aa: [usize; n],
    };
    let mut bts = BTreeSet::new();
    for a in aa.iter().copied() {
        bts.insert(a);
    }
    let min = bts.first().copied().unwrap();
    let max = bts.last().copied().unwrap();
    for i in min..max {
        if !bts.contains(&i) {
            println!("{i}");
            return;
        }
    }
}
