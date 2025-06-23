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
        x: usize,
        y: usize,
    };
    let cnt = (1..=6).cartesian_product(1..=6)
        .filter(|&(a, b)| x <= a + b || y <= a.abs_diff(b))
        .count();
    let rs = cnt as f64 / 36.0;
    println!("{rs}");
}
