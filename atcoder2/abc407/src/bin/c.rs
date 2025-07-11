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
        s: Bytes,
    };
    let mut aa = s.iter().map(|&c| (c - b'0') as usize).collect_vec();
    let mut sub = 0;
    let mut rs = 0;
    while let Some(a) = aa.pop() {
        let b = (10 + a - sub) % 10;
        rs += b + 1;
        sub = (sub + b) % 10;
    }
    println!("{rs}");
}
