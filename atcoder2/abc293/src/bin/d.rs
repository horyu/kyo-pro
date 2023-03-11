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
        aabbccdd: [(Usize1, char, Usize1, char); m],
    };
    let mut dsu = ac_library_rs::Dsu::new(n);
    let mut circle = 0;
    for (a, _b, c, _d) in aabbccdd {
        if dsu.same(a, c) {
            circle += 1;
        }
        dsu.merge(a, c);
    }
    println!("{circle} {}", dsu.groups().len() - circle);
}
