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
        ss: [isize; a],
        tt: [isize; b],
        xx: [isize; q],
    };
    let ss = BTreeSet::from_iter(chain!([std::isize::MIN >> 2], ss, [std::isize::MAX >> 2]));
    let tt = BTreeSet::from_iter(chain!([std::isize::MIN >> 2], tt, [std::isize::MAX >> 2]));
    for x in xx {
        let ss = [
            ss.range(..=x).next_back().copied().unwrap(),
            ss.range(x..).next().copied().unwrap(),
        ];
        let tt = [
            tt.range(..=x).next_back().copied().unwrap(),
            tt.range(x..).next().copied().unwrap(),
        ];
        let mut rs = std::usize::MAX;
        for (s, t) in ss.into_iter().cartesian_product(tt) {
            rs = rs.min(x.abs_diff(s) + s.abs_diff(t));
            rs = rs.min(x.abs_diff(t) + t.abs_diff(s));
        }
        println!("{rs}");
    }
}
