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
        aa: [usize; n],
    };
    let mut ft = ac_library::FenwickTree::new(1e6 as usize + 1, 0usize);
    for a in aa.iter().copied() {
        ft.add(a, a);
    }
    let mut rrss = vec![];
    for a in aa.iter().copied() {
        rrss.push(ft.sum((a + 1)..));
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
