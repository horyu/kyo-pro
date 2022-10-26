#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let counts = aa.clone().into_iter().counts();
    let mut bb = aa;
    bb.sort_unstable();
    bb.dedup();
    let bblen = bb.len();
    let mut rs = vec![0; n];
    for (i, &b) in bb.iter().enumerate() {
        rs[bblen - 1 - i] = *counts.get(&b).unwrap();
    }
    for r in rs {
        println!("{r}");
    }
}
