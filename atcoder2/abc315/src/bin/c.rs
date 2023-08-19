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
        n: usize,
        ffss: [(usize, usize); n],
    };
    let mut mm = multimap::MultiMap::new();
    for (f, s) in ffss {
        mm.insert(f, s);
    }
    let mut rs = 0;
    let mut max = 0;
    for (_, mut ss) in mm {
        ss.sort_unstable();
        ss.reverse();
        if 2 <= ss.len() {
            rs = rs.max(ss[0] + ss[1] / 2)
        }
        rs = rs.max(max + ss[0]);
        max = max.max(ss[0]);
    }
    println!("{rs}");
}
