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
        c: usize,
        ddd: [[usize; c]; c],
        ccc: [[Usize1; n]; n],
    };
    let size = 3usize;
    let mut counters = vec![counter::Counter::<usize>::new(); size];
    for (i, cc) in ccc.into_iter().enumerate() {
        for (j, c) in cc.into_iter().enumerate() {
            counters[(i + j) % 3][&c] += 1usize;
        }
    }
    let mut rs = usize::MAX;
    for cc in (0..c).permutations(3) {
        let mut tmp = 0;
        for (counter, c) in izip!(&counters, cc) {
            for (&k, &v) in counter {
                tmp += ddd[k][c] * v;
            }
        }
        rs = rs.min(tmp);
    }
    println!("{rs}");
}
