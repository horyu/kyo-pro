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
        aacc: [(usize, usize); n],
    };
    let rs = aacc
        .into_iter()
        .into_group_map_by(|ac| ac.1)
        .into_values()
        .map(|aacc| aacc.into_iter().map(|ac| ac.0).min().unwrap())
        .max()
        .unwrap();
    println!("{rs}");
}
