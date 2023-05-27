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
        m: usize,
        aaa: [[Usize1; n]; m],
    };
    let mut hs: HashSet<(usize, usize)> = (0..n).tuple_combinations().collect();
    for aa in aaa {
        for (ax, ay) in aa.into_iter().tuple_windows() {
            hs.remove(&(ax.min(ay), ax.max(ay)));
        }
    }
    let rs = hs.len();
    println!("{rs}");
}