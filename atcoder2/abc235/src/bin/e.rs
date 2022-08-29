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
        m: usize,
        q: usize,
        aabbcc: [(Usize1, Usize1, usize); m],
        uuvvww: [(Usize1, Usize1, usize); q],
    };
    let mut rs = vec![false; q];
    let mut ee = vec![];
    for abc in aabbcc {
        ee.push((std::usize::MAX, abc));
    }
    for (i, abc) in uuvvww.into_iter().enumerate() {
        ee.push((i, abc));
    }
    ee.sort_unstable_by_key(|iabc| iabc.1 .2);
    let mut uf = UnionFind::new(n);
    for (i, (a, b, _c)) in ee {
        if !uf.equiv(a, b) {
            if i == std::usize::MAX {
                uf.union(a, b);
            } else {
                rs[i] = true;
            }
        }
    }
    let rs = rs
        .into_iter()
        .map(|tf| ["No", "Yes"][tf as usize])
        .join("\n");
    println!("{rs}");
}
