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
        aabb: [(Usize1, Usize1); m],
    };
    let mut rs = vec![n * (n - 1) / 2; m];
    let mut dsu = ac_library_rs::Dsu::new(n);
    let mut sub = 0;
    for (i, (a, b)) in aabb.into_iter().enumerate().rev() {
        rs[i] -= sub;

        if !dsu.same(a, b) {
            sub += dsu.size(a) * dsu.size(b);
            dsu.merge(a, b);
        }
    }
    for rs in rs {
        println!("{rs}");
    }
}
