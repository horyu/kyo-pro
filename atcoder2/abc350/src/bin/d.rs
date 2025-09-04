#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        m: usize,
        aabb: [(Usize1, Usize1); m],
    };
    let mut dsu = ac_library::Dsu::new(n);
    for (a, b) in aabb {
        dsu.merge(a, b);
    }
    let mut rs = 0;
    for vv in dsu.groups() {
        let len = vv.len();
        rs += len * len.saturating_sub(1) / 2;
    }
    rs -= m;
    println!("{rs}");
}
