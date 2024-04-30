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
        a: usize,
        b: u32,
        c: usize,
    };
    // log2 a < b * log2 c
    // log2 a < log2 c^b
    // a < c^b
    let tf = a < c.saturating_pow(b);
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
