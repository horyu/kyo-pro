#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aabb: [(usize, usize); n],
    };
    let mut vv = vec![0isize; 1100000];
    for (a, b) in aabb {
        vv[a] += 1;
        vv[b + 1] -= 1;
    }
    let mut rs = 0isize;
    let mut crr = 0isize;
    for v in vv {
        crr += v;
        rs = rs.max(crr);
    }
    println!("{rs}");
}
