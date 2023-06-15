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
        m: usize,
        xxyyzz: [(Usize1, Usize1, usize); m],
    };
    // 121
    // 12: 1 : 21
    // 21: 1 : 12
    // 11: 0 : 22
    let mut dsu = ac_library::Dsu::new(n);
    for (x, y, z) in xxyyzz.iter().copied() {
        dsu.merge(x, y);
    }
    let rs = dsu.groups().len();
    println!("{rs}");
}
