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
        aabbccdd: [(usize, usize, usize, usize); n],
    };
    let mut hs = HashSet::new();
    for (a, b, c, d) in aabbccdd {
        for x in a..b {
            for y in c..d {
                hs.insert((x, y));
            }
        }
    }
    let rs = hs.len();
    println!("{rs}");
}
