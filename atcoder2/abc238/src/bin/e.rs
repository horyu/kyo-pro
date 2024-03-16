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
        q: usize,
        llrr: [(usize, usize); q],
    };
    let mut dsu = ac_library::Dsu::new(n + 1);
    for (l, r) in llrr.iter().copied() {
        dsu.merge(l - 1, r);
    }
    let rs = ["No", "Yes"][dsu.same(0, n) as usize];
    println!("{rs}");
}
