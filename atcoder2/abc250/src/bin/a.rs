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
        h: usize,
        w: usize,
        r: usize,
        c: usize,
    };
    let mut rs = 0;
    if h != 1 {
        rs += if r == 1 || r == h { 1 } else { 2 };
    }
    if w != 1 {
        rs += if c == 1 || c == w { 1 } else { 2 };
    }
    println!("{rs}");
}
