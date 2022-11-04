#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut aabb: [(isize, isize); n],
        mut ccdd: [(isize, isize); n],
    };
    ccdd.sort_unstable_by(|x, y| x.0.cmp(&y.0).then(x.1.cmp(&y.1)));
    let mut rs = 0;
    for (c, d) in ccdd {
        if let Some(i) = (0..aabb.len())
            .filter(|&i| {
                let (a, b) = aabb[i];
                a < c && b < d
            })
            .max_by_key(|&i| aabb[i].1)
        {
            rs += 1;
            aabb.swap_remove(i);
        };
    }

    println!("{rs}");
}
