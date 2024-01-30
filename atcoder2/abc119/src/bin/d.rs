#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
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

#[allow(dead_code)]
fn main2() {
    input! {
        a: usize,
        b: usize,
        q: usize,
        ss: [usize; a],
        tt: [usize; b],
        xx: [usize; q],
    };
    let sbts = BTreeSet::from_iter(ss.iter().copied());
    let tbts = BTreeSet::from_iter(tt.iter().copied());
    for x in xx {
        let mut rs = std::usize::MAX;
        for [bts0, bts1] in [[&sbts, &tbts], [&tbts, &sbts]] {
            for y in [
                bts0.range(..x).next_back().copied(),
                bts0.range(x..).next().copied(),
            ]
            .into_iter()
            .flatten()
            {
                for z in [
                    bts1.range(..y).next_back().copied(),
                    bts1.range(y..).next().copied(),
                ]
                .into_iter()
                .flatten()
                {
                    rs = rs.min(x.abs_diff(y) + y.abs_diff(z));
                }
            }
        }
        println!("{rs}");
    }
}
