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
        q: usize,
        xxyy: [(isize, isize); n],
        qq: [Usize1; q],
    };
    let mut min_a = std::isize::MAX;
    let mut max_a = std::isize::MIN;
    let mut min_b = std::isize::MAX;
    let mut max_b = std::isize::MIN;

    let mut aabb = vec![];
    for (x, y) in xxyy {
        let a = x + y;
        let b = x - y;
        aabb.push((a, b));
        min_a = min_a.min(a);
        max_a = max_a.max(a);
        min_b = min_b.min(b);
        max_b = max_b.max(b);
    }

    for q in qq {
        let (a, b) = aabb[q];
        let rs = [
            a.abs_diff(min_a),
            a.abs_diff(max_a),
            b.abs_diff(min_b),
            b.abs_diff(max_b),
        ]
        .iter()
        .max()
        .copied()
        .unwrap();
        println!("{rs}");
    }
}
