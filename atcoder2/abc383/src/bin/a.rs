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
        ttvv: [(usize, usize); n],
    };
    let mut rs = 0usize;
    for now in 1..=ttvv[n - 1].0 {
        rs = rs.saturating_sub(1);
        if let Some((t, v)) = ttvv.iter().find(|(t, v)| *t == now) {
            rs += v;
        }
    }
    println!("{rs}");
}
