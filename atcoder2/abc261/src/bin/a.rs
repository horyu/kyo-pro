#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize,
    };
    let mut arr = [0; 101];
    for i in (l1 + 1)..=r1 {
        arr[i] += 1;
    }
    for i in (l2 + 1)..=r2 {
        arr[i] += 1;
    }
    let rs = arr.into_iter().filter(|&a| a == 2).count();
    println!("{rs}");
}
