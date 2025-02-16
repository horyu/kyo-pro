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
        xx: [usize; 4]
    };
    let vv = xx
        .into_iter()
        .counts()
        .into_values()
        .sorted_unstable()
        .collect_vec();
    let tf = vv == [2, 2] || vv == [1, 3];
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
