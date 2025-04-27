#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
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
    let rs = aa
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, i as isize))
        .into_group_map()
        .into_iter()
        .filter(|(_, ii)| ii.len() == 1)
        .max()
        .map(|(_, ii)| ii[0] + 1)
        .unwrap_or(-1);
    println!("{rs}");
}
