#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
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
    for (a, b) in aabb {
        if b <= a {
            println!("{}", a - b);
            continue;
        }
        let mut rs = std::usize::MAX;
        for x in 0usize..=2_000_000 {
            let ax = a + x;
            let by = b.div_ceil(ax) * ax;
            let y = by - b;
            rs = rs.min(x + y);

        }
        println!("{rs}");
    }
}
