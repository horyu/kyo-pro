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
    let mut ft = ac_library::FenwickTree::new(n, 0);
    let mut rs = 0;
    for a in aa.iter().copied() {
        rs += ft.sum(a..n);
        ft.add(a, 1);
    }
    for a in aa.iter().copied() {
        println!("{rs}");
        rs = rs + n - 1 - a * 2;
    }
}
