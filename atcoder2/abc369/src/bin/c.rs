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
        aa: [isize; n],
    };
    let mut rs = n;
    for g in aa
        .into_iter()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .group_by(|&x| x)
        .into_iter()
    {
        let size = g.1.count();
        rs += (1 + size) * size / 2;
    }
    println!("{rs}");
}
