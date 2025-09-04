#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
#![feature(unsigned_is_multiple_of)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        y: usize,
    };
    let rs = if !y.is_multiple_of(4) {
        365
    } else if !y.is_multiple_of(100) {
        366
    } else if !y.is_multiple_of(400) {
        365
    } else {
        366
    };
    println!("{rs}");
}
