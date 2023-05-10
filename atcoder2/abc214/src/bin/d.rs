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
        mut uuvvww: [(Usize1, Usize1, usize); n - 1],
    };
    uuvvww.sort_unstable_by_key(|uvw| uvw.2);
    let mut dsu = ac_library::Dsu::new(n);
    let mut rs = 0;
    for (u, v, w) in uuvvww {
        rs += w * dsu.size(u) * dsu.size(v);
        dsu.merge(u, v);
    }
    println!("{rs}");
}
