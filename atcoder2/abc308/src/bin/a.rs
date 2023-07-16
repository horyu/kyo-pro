#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        ss: [usize; 8],
    };
    let tf = ss.iter().copied().tuple_windows().all(|(a, b)| a <= b)
        && ss.iter().copied().all(|s| (100..=675).contains(&s))
        && ss.iter().copied().all(|s| s % 5 == 0);
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}