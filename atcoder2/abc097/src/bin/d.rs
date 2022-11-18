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
        pp: [Usize1; n],
        xxyy: [(Usize1, Usize1); m],
    };
    let mut dsu = ac_library_rs::Dsu::new(n);
    for &(x, y) in &xxyy {
        dsu.merge(x, y);
    }
    let mut rs = 0usize;
    for gg in dsu.groups() {
        let hs: HashSet<usize> = gg.into_iter().collect();
        for &i in &hs {
            if hs.contains(&pp[i]) {
                rs += 1;
            }
        }
    }
    println!("{rs}");
}
