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
        aabb: [(usize, usize); n]
    };
    let a_sum = aabb.iter().map(|&(a, _)| a).sum::<usize>();
    let (a, b) = aabb.into_iter().max_by_key(|(a, b)| b - a).unwrap();
    let rs = a_sum - a + b;
    println!("{rs}");
}
