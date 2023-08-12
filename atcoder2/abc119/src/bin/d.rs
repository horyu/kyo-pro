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
        a: usize,
        b: usize,
        q: usize,
        ss: [usize; a],
        tt: [usize; b],
        xx: [usize; q],
    };
    let sbts = BTreeSet::from_iter(ss);
    let tbts = BTreeSet::from_iter(tt);
    for x in xx {
        let sl = sbts.range(..x).next_back().copied().unwrap_or(1 << 60);
        let sr = sbts.range(x..).next().copied().unwrap_or(1 << 60);
        let tl = tbts.range(..x).next_back().copied().unwrap_or(1 << 60);
        let tr = tbts.range(x..).next().copied().unwrap_or(1 << 60);
        let mut rs = usize::MAX;
        for (s, t) in iproduct!([sl, sr], [tl, tr]) {
            rs = rs.min(x.abs_diff(s) + s.abs_diff(t));
            rs = rs.min(x.abs_diff(t) + t.abs_diff(s));
        }
        println!("{rs}");
    }
}
